pub mod records;
use async_trait::async_trait;
use records::{
    // AuthorRecord, SeriesAndVolumeRecord,
    BookRecord,
};
use shared_core::domain::{
    metadata::{BookContributor, BookMetadata, BookSeries},
    repository::BookItem,
};
use shared_core::ports::repository::{
    BookRepositoryPort,
    FetchError,
    InsertError,
    OpenRepositoryError, // UpdateError,
};
use sqlx::{Sqlite, SqliteConnection, SqlitePool, Transaction, sqlite::SqliteConnectOptions};
use std::path::{Path, PathBuf};

pub struct Database {
    /// pool used to execute queries in the `SQLite` database
    pool: SqlitePool,
}

#[async_trait]
impl BookRepositoryPort for Database {
    #[inline]
    async fn close(&self) {
        self.pool.close().await;
    }

    #[inline]
    async fn fetch_all_books(&self) -> Result<Vec<BookItem>, FetchError> {
        let books: Vec<BookRecord> = sqlx::query_as(
            "WITH series_info AS (
                SELECT 
                    bsl.book, 
                    Json_group_array(
                        Json_object(
                            'series', s.NAME, 
                            'sort', s.sort, 
                            'volume', bsl.entry,
                            'goodreads_id', s.GOODREADS_ID
                        )
                    ) series_and_volume 
                FROM 
                    series AS s 
                    JOIN books_series_link bsl ON bsl.series = s.id 
                GROUP BY 
                    bsl.book
            ), 
            authors_info AS (
                SELECT 
                    Json_group_array(Json_object(
                        'name', a.NAME,
                        'sort', a.SORT,
                        'goodreads_id', a.GOODREADS_ID
                    )) authors,
                    bal.book 
                FROM 
                    authors AS a 
                    JOIN books_authors_link bal ON a.id = bal.author 
                GROUP BY 
                    bal.book
            ) 
            SELECT 
                id AS book_id, 
                title, 
                sort, 
                date_added, 
                date_published, 
                last_modified AS date_modified, 
                number_of_pages, 
                goodreads_id, 
                authors, 
                CASE WHEN series_and_volume IS NULL 
                OR Trim(series_and_volume) = '' THEN '[]' WHEN Json_valid
                    (series_and_volume) = 1 THEN series_and_volume ELSE '[]' END AS 
                    series_and_volume 
            FROM 
                books 
                LEFT JOIN series_info ON series_info.book = books.id 
                JOIN authors_info ON authors_info.book = books.id 
            ORDER BY 
                books.date_added ASC;
        ",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|error| FetchError::Generic {
            message: error.to_string(),
        })?;
        Ok(books.into_iter().map(Into::into).collect())
    }

    #[inline]
    async fn try_fetch_author_sort(&self, author_name: &str) -> Result<Option<String>, FetchError> {
        let row = sqlx::query!("SELECT sort FROM authors WHERE name LIKE ?", author_name)
            .fetch_optional(&self.pool)
            .await
            .map_err(|error| FetchError::Generic {
                message: error.to_string(),
            })?;

        Ok(row.map(|row| row.sort))
    }

    #[inline]
    async fn try_fetch_series_sort(
        &self,
        series_title: &str,
    ) -> Result<Option<String>, FetchError> {
        let row = sqlx::query!("SELECT sort FROM series WHERE name LIKE ?", series_title)
            .fetch_optional(&self.pool)
            .await
            .map_err(|error| FetchError::Generic {
                message: error.to_string(),
            })?;

        Ok(row.map(|row| row.sort))
    }

    #[inline]
    async fn insert_book(&self, book: BookMetadata) -> Result<(), InsertError> {
        let mut tx: Transaction<'_, Sqlite> =
            self.pool
                .begin()
                .await
                .map_err(|error| InsertError::Unavailable {
                    message: error.to_string(),
                })?;

        let book_goodreads_id = book.goodreads_id.clone();
        let number_of_pages = book.number_of_pages;
        let sort = get_title_sort(&book.title);

        let book_id_res: Result<i64, sqlx::Error> = sqlx::query_scalar!(
            r#"
            INSERT INTO books (
                title,
                sort,
                date_published,
                number_of_pages,
                goodreads_id
            )
            VALUES (?, ?, ?, ?, ?)
                RETURNING id; 
        "#,
            book.title,
            sort,
            book.publication_date,
            number_of_pages,
            book_goodreads_id.0
        )
        .fetch_one(&mut *tx)
        .await;

        // If book was inserted successfully, fetch its internal ID, otherwise return early and
        // rollback previous SQL query
        let book_id = match book_id_res {
            Ok(id) => id,
            Err(error) => {
                if is_sqlite_unique_violation(&error) {
                    tx.rollback()
                        .await
                        .map_err(|insert_error| InsertError::Unavailable {
                            message: insert_error.to_string(),
                        })?;
                    return Err(InsertError::Conflict {
                        goodreads_id: book.goodreads_id.clone(),
                    });
                }
                return Err(InsertError::Unavailable {
                    message: String::from("failed to insert book"),
                });
            }
        };

        // handle authors
        self.insert_authors(&mut tx, book.contributors, book_id)
            .await?;

        // handle series
        self.insert_series(&mut tx, book.series, book_id).await?;
        tx.commit()
            .await
            .map_err(|error| InsertError::Unavailable {
                message: error.to_string(),
            })?;

        Ok(())
    }

    // async fn update_book(&self, book: BookItem) -> Result<(), UpdateError> {}

    // async fn update_series(&self, series: SeriesAndVolumeItem) -> Result<(), UpdateError> {}

    // async fn update_author(&self, author: AuthorItem) -> Result<(), UpdateError> {}
}

impl Database {
    /// Tries to instantiate an instance that connects to an existing `SQLite` database
    ///
    /// # Errors
    /// Fails if the path doesn't exist or if running the migration fails
    #[inline]
    pub async fn open(path: &Path) -> Result<Self, OpenRepositoryError> {
        let options = SqliteConnectOptions::new()
            .foreign_keys(true)
            .filename(path);
        let pool =
            SqlitePool::connect_with(options)
                .await
                .map_err(|_err| OpenRepositoryError::Path {
                    path: PathBuf::from(path),
                })?;
        sqlx::migrate!()
            .run(&pool)
            .await
            .map_err(|_err| OpenRepositoryError::Initialization)?;

        Ok(Self { pool })
    }

    /// Insert all authors associated with a book
    ///
    /// # Errors
    /// Fails if the new author cannot be created, if the repository is unavailable or if the link
    /// between book and author cannot be created
    async fn insert_authors(
        &self,
        conn: &mut SqliteConnection,
        authors: Vec<BookContributor>,
        book_id: i64,
    ) -> Result<(), InsertError> {
        for author_record in &authors {
            let author_goodreads_id = author_record.goodreads_id.clone();
            let author_sort = self
                .try_fetch_author_sort(&author_record.name)
                .await
                .map_err(|error| InsertError::Unavailable {
                    message: error.to_string(),
                })?
                .map_or_else(|| get_name_sort(&author_record.name), |string| string);
            let author_id: i64 = sqlx::query!(
                r#"
                    INSERT INTO authors(name, sort, goodreads_id)
                    VALUES (?, ?, ?)
                    ON CONFLICT(goodreads_id) DO UPDATE SET
                        name = excluded.name,
                        sort = excluded.sort
                    RETURNING id;
                "#,
                author_record.name,
                author_sort,
                author_goodreads_id.0
            )
            .fetch_one(&mut *conn)
            .await
            .map_err(|error| InsertError::Entity {
                entity: "author".into(),
                name: author_record.name.clone(),
                message: error.to_string(),
            })?
            .id;

            sqlx::query!(
                r#"
                INSERT OR IGNORE INTO books_authors_link(book, author)
                VALUES (?1, ?2);
            "#,
                book_id,
                author_id
            )
            .execute(&mut *conn)
            .await
            .map_err(|error| InsertError::Entity {
                entity: "book_author_link".into(),
                name: author_record.name.clone(),
                message: error.to_string(),
            })?;
        }
        Ok(())
    }

    /// Insert all series associated with a book
    ///
    /// # Errors
    /// Fails if the new series cannot be created, if the repository is unavailable or if the link
    /// between book and series cannot be created
    async fn insert_series(
        &self,
        conn: &mut SqliteConnection,
        series: Vec<BookSeries>,
        book_id: i64,
    ) -> Result<(), InsertError> {
        for sav in &series {
            let sav_goodreads_id = sav.goodreads_id.clone();
            let series_sort = self
                .try_fetch_series_sort(&sav.title)
                .await
                .map_err(|error| InsertError::Unavailable {
                    message: error.to_string(),
                })?
                .map_or_else(|| get_title_sort(&sav.title), |string| string);
            let series_id: i64 = sqlx::query!(
                r#"
                INSERT INTO series(name, sort, goodreads_id)
                VALUES (?, ?, ?)
                ON CONFLICT(goodreads_id) DO UPDATE SET
                    name = EXCLUDED.name,
                    sort = EXCLUDED.sort
                RETURNING id;
            "#,
                sav.title,
                series_sort,
                sav_goodreads_id.0
            )
            .fetch_one(&mut *conn)
            .await
            .map_err(|error| InsertError::Entity {
                entity: "series".into(),
                name: sav.title.clone(),
                message: error.to_string(),
            })?
            .id;

            sqlx::query!(
                r#"
                INSERT INTO books_series_link(book, series, entry)
                VALUES (?, ?, ?)
            "#,
                book_id,
                series_id,
                sav.number
            )
            .execute(&mut *conn)
            .await
            .map_err(|error| InsertError::Entity {
                entity: "books_series_link".into(),
                name: sav.title.clone(),
                message: error.to_string(),
            })?;
        }
        Ok(())
    }
}

/// Checks a returned `sqlx` error to see whether it is because of a unique constraint being
/// violated by checking the error's attached message.
#[allow(
    clippy::pattern_type_mismatch,
    reason = "False positive, this is the idiomatic pattern"
)]
fn is_sqlite_unique_violation(error: &sqlx::Error) -> bool {
    // Check for unique violation by searching for matching text in error message
    if let sqlx::Error::Database(db_err) = error {
        db_err.message().contains("UNIQUE constraint failed")
    } else {
        false
    }
}

#[must_use]
#[inline]
pub fn get_name_sort(author_name: &str) -> String {
    // Takes the full name of an author and produces a string according to which the name should
    // be sorted. General logic: Sort by last "word" in name and comma-separate it from everything
    // else in the name, e.g. `Guy Le Best => Best, Guy Le`
    let tokens = author_name.split_whitespace().collect::<Vec<&str>>();

    match tokens.as_slice().split_last() {
        None => String::new(),
        Some((only, &[])) => (*only).to_owned(),
        Some((last, rest)) => format!("{}, {}", last, rest.join(" ")),
    }
}

#[must_use]
#[inline]
pub fn get_title_sort(title: &str) -> String {
    // Required patterns:
    // the everythingelse -> everythingelse, the e.g. The Hobbit
    // a everythingelse -> everythingelse, a e.g. A Game of Thrones
    // an everythingelse -> everythingelse, an e.g. An Echo of Thigns to Come
    if let Some(prefix) = title.split_whitespace().next() {
        if ["A", "An", "The"].contains(&prefix) {
            let remainder = title.replace(prefix, "");
            let trimmed_remainder = remainder.trim();
            return format!("{trimmed_remainder}, {prefix}");
        }
    }
    title.to_owned()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "Tests")]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use pretty_assertions::assert_eq;
    use shared_core::domain::repository::GoodreadsId;
    use std::fs;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn firstname_lastname() {
        let names = [
            String::from("Brandon Sanderson"),
            String::from("Robert Jordan"),
            String::from("Tad Williams"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        let expected = vec![
            String::from("Sanderson, Brandon"),
            String::from("Jordan, Robert"),
            String::from("Williams, Tad"),
        ];

        assert_eq!(results, expected);
    }

    #[test]
    fn firstname_middlename_lastname() {
        let names = [
            String::from("Guy Gavriel Kay"),
            String::from("Orson Scott Card"),
            String::from("Justin Lee Anderson"),
        ];

        let expected = vec![
            String::from("Kay, Guy Gavriel"),
            String::from("Card, Orson Scott"),
            String::from("Anderson, Justin Lee"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn firstname_m_lastname() {
        let names = [
            String::from("Michael J. Sullivan"),
            String::from("Arthur C. Clarke"),
            String::from("Philip K. Dick"),
            String::from("Ursula K. Le Guin"),
        ];

        let expected = vec![
            String::from("Sullivan, Michael J."),
            String::from("Clarke, Arthur C."),
            String::from("Dick, Philip K."),
            String::from("Guin, Ursula K. Le"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn f_m_lastname() {
        let names = [
            String::from("R. R. Virdi"),
            String::from("S. A. Chakraborty"),
            String::from("M. L. Wang"),
        ];

        let expected = vec![
            String::from("Virdi, R. R."),
            String::from("Chakraborty, S. A."),
            String::from("Wang, M. L."),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn firstname_m_m_lastname() {
        let names = [
            String::from("James S. A. Corey"),
            String::from("George R. R. Martin"),
        ];

        let expected = vec![
            String::from("Corey, James S. A."),
            String::from("Martin, George R. R."),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn f_middlename_lastname() {
        let names = [
            String::from("R. Scott Bakker"),
            String::from("F. Scott Fitzgerald"),
        ];

        let expected = vec![
            String::from("Bakker, R. Scott"),
            String::from("Fitzgerald, F. Scott"),
        ];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn f_m_m_lastname() {
        let name = "J. R. R. Tolkien";

        let expected = String::from("Tolkien, J. R. R.");
        let result = get_name_sort(name);

        assert_eq!(expected, result);
    }

    #[test]
    fn singlename() {
        let names = [String::from("Baoshu"), String::from("Madonna")];

        let expected = vec![String::from("Baoshu"), String::from("Madonna")];

        let results: Vec<String> = names.iter().map(|name| get_name_sort(name)).collect();

        assert_eq!(expected, results);
    }

    #[test]
    fn titles() {
        let titles = [
            String::from("A Game of Thrones"),
            String::from("An Echo of Things to Come"),
            String::from("The Hobbit"),
            String::from("Neverwhere"),
            String::from("I Am Not A Serial Killer"),
            String::from("Mr Monster"),
            String::from("The Hero of Ages"),
            String::from("The Great Hunt"),
        ];

        let expected = vec![
            String::from("Game of Thrones, A"),
            String::from("Echo of Things to Come, An"),
            String::from("Hobbit, The"),
            String::from("Neverwhere"),
            String::from("I Am Not A Serial Killer"),
            String::from("Mr Monster"),
            String::from("Hero of Ages, The"),
            String::from("Great Hunt, The"),
        ];

        let results: Vec<String> = titles.iter().map(|title| get_title_sort(title)).collect();

        assert_eq!(expected, results);
    }

    #[tokio::test]
    async fn duplicate_book() {
        let _temp_db = File::create("temp.db").unwrap();
        let db = Database::open(Path::new("temp.db")).await.unwrap();

        let book = BookMetadata::new(
            "The Hobbit",
            Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1937, 9, 21).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
            )),
            vec![BookContributor::new(
                "J. R. R. Tolkien",
                "Author",
                GoodreadsId::new(656_983),
            )],
            vec![BookSeries::new(
                "Middle Earth",
                1.0,
                GoodreadsId::new(66175),
            )],
            Some(366),
            Some(String::from(
                "https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1546071216i/5907.jpg",
            )),
            GoodreadsId::new(5907),
        );

        db.insert_book(book.clone()).await.unwrap();

        // book is now in database already, assert that adding it a second time fails
        let result = db.insert_book(book).await;
        assert_eq!(
            result,
            Err(InsertError::Conflict {
                goodreads_id: GoodreadsId::new(5907)
            })
        );
        db.close().await;
        if Path::exists(Path::new("temp.db")) {
            fs::remove_file("temp.db").unwrap();
        }
    }
}
