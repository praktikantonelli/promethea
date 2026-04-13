use std::path::PathBuf;

use async_trait::async_trait;
use shared_core::ports::repository::{
    FetchError, InsertBookError, OpenRepositoryError, UpdateError,
};
use shared_core::{
    domain::{
        metadata::BookMetadata, records::AuthorRecord, records::BookRecord,
        records::SeriesAndVolumeRecord,
    },
    ports::repository::BookRepositoryPort,
};
use sqlx::{Sqlite, SqlitePool, Transaction, sqlite::SqliteConnectOptions};

pub struct Database {
    pool: SqlitePool,
}

#[async_trait]
impl BookRepositoryPort for Database {
    async fn close(&self) {
        self.pool.close().await;
    }

    async fn fetch_all_books(&self) -> Result<Vec<BookRecord>, FetchError> {
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
        Ok(books)
    }

    async fn try_fetch_author_sort(&self, author_name: &str) -> Result<Option<String>, FetchError> {
        let sort = sqlx::query!("SELECT sort FROM authors WHERE name LIKE ?", author_name)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| FetchError::Generic {
                message: error.to_string(),
            })?
            .sort;

        Ok(Some(sort))
    }

    async fn try_fetch_series_sort(
        &self,
        series_title: &str,
    ) -> Result<Option<String>, FetchError> {
        let sort = sqlx::query!("SELECT sort FROM series WHERE name LIKE ?", series_title)
            .fetch_one(&self.pool)
            .await
            .map_err(|error| FetchError::Generic {
                message: error.to_string(),
            })?
            .sort;
        Ok(Some(sort))
    }

    async fn insert_book(&self, book: BookMetadata) -> Result<(), InsertBookError> {
        let mut tx: Transaction<'_, Sqlite> = self.pool.begin().await?;

        let book_goodreads_id = book.goodreads_id;
        let number_of_pages = book.number_of_pages;

        let book_id_res: Result<i64, sqlx::Error> = sqlx::query_scalar!(
            r#"
            INSERT INTO books (
                title,
                sort,
                date_added,
                date_published,
                number_of_pages,
                goodreads_id
            )
            VALUES (?, ?, ?, ?, ?, ?)
                RETURNING id; 
        "#,
            book.title,
            book.sort,
            book.date_added,
            book.date_published,
            number_of_pages,
            book_goodreads_id
        )
        .fetch_one(&mut *tx)
        .await;

        // If book was inserted successfully, fetch its internal ID, otherwise return early and
        // rollback previous SQL query
        let book_id = match book_id_res {
            Ok(id) => id,
            Err(error) => {
                if is_sqlite_unique_violation(&error) {
                    tx.rollback().await?;
                    return Err(InsertBookError::Conflict {
                        goodreads_id: book.goodreads_id.unwrap(),
                    });
                }
                return Err(InsertBookError::Unavailable);
            }
        };

        // handle authors
        for author_record in &book.authors {
            let author_goodreads_id = author_record.goodreads_id;
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
                author_record.sort,
                author_goodreads_id
            )
            .fetch_one(&mut *tx)
            .await?
            .id;

            sqlx::query!(
                r#"
                INSERT OR IGNORE INTO books_authors_link(book, author)
                VALUES (?1, ?2);
            "#,
                book_id,
                author_id
            )
            .execute(&mut *tx)
            .await?;
        }

        // handle series
        for sav in &book.series_and_volume {
            let sav_goodreads_id = sav.goodreads_id;
            let series_id: i64 = sqlx::query!(
                r#"
                INSERT INTO series(name, sort, goodreads_id)
                VALUES (?, ?, ?)
                ON CONFLICT(goodreads_id) DO UPDATE SET
                    name = EXCLUDED.name,
                    sort = EXCLUDED.sort
                RETURNING id;
            "#,
                sav.series,
                sav.sort,
                sav_goodreads_id
            )
            .fetch_one(&mut *tx)
            .await?
            .id;

            sqlx::query!(
                r#"
                INSERT INTO books_series_link(book, series, entry)
                VALUES (?, ?, ?)
            "#,
                book_id,
                series_id,
                sav.volume
            )
            .execute(&mut *tx)
            .await?;
        }
        tx.commit().await?;

        Ok(())
    }

    async fn update_book(&self, book: BookRecord) -> Result<(), UpdateError> {}

    async fn update_series(&self, series: SeriesAndVolumeRecord) -> Result<(), UpdateError> {}

    async fn update_author(&self, author: AuthorRecord) -> Result<(), UpdateError> {}
}

impl Database {
    async fn open(path: &std::path::Path) -> Result<Self, OpenRepositoryError> {
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
