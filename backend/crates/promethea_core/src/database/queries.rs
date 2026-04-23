use crate::database::types::{BookRecord, InsertBookError};
use sqlx::{Sqlite, SqlitePool, Transaction, sqlite::SqliteConnectOptions};
use std::path::Path;

pub struct Db {
    /// Used to execute queries with a persistent pool that is cheaply clonable.
    pool: SqlitePool,
}

impl Db {
    /// Initialization function for database connection.
    /// # Errors
    /// This function fails if the `SQLite` database file cannot be accessed or if there's a problem
    /// running the migrations on it.
    #[allow(
        clippy::missing_inline_in_public_items,
        reason = "Called once at start of program"
    )]
    pub async fn init(path: &Path) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .foreign_keys(true)
            .filename(path);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    #[allow(
        clippy::missing_inline_in_public_items,
        reason = "Called once at end of program"
    )]
    pub async fn close(&self) {
        self.pool.close().await;
    }

    /// Function to fetch all books in the library. Performs multiple joins
    /// # Errors
    /// This function should generally NOT return an error, but if there happens to be a problem
    /// with the database, that error is propagated upwards
    #[allow(
        clippy::missing_inline_in_public_items,
        reason = "Large function, called only when table updates"
    )]
    pub async fn fetch_books_query(&self) -> Result<Vec<BookRecord>, sqlx::Error> {
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
        .await?;
        Ok(books)
    }

    /// Queries the database to try and find an existing record on how to sort an author's name.
    /// This is done so if an author requires a special sorting rule, that rule only has to be
    /// defined once.
    /// # Errors
    /// This function errors only if there is a problem communicating with the database.
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub async fn try_fetch_author_sort(&self, name: &str) -> Result<Option<String>, sqlx::Error> {
        let sort = sqlx::query!("SELECT sort FROM authors WHERE name LIKE ?", name)
            .fetch_one(&self.pool)
            .await?
            .sort;

        Ok(Some(sort))
    }

    /// Queries the database to try and find an existing record on how to sort a series' name.
    /// This is done so if a series requires a special sorting rule, that rule only has to be
    /// defined once.
    /// # Errors
    /// This function errors only if there is a problem communicating with the database.
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub async fn try_fetch_series_sort(&self, name: &str) -> Result<Option<String>, sqlx::Error> {
        let sort = sqlx::query!("SELECT sort FROM series WHERE name LIKE ?", name)
            .fetch_one(&self.pool)
            .await?
            .sort;

        Ok(Some(sort))
    }

    /// Takes a `BookRecord` object and tries to insert it into the library of owned books. The
    /// function works as follows:
    /// 1. Insert just the book (title, sort, date added, date published, last modified, number of
    ///    pages, goodreads id).
    /// 2. Fetch the book's ID (succeeds if just created, otherwise warns about the book already
    ///    existing).
    /// 3. Insert the author(s)' name(s). Existing author records don't matter.
    /// 4. Fetch author ID(s)
    /// 5. Insert series. Existing series don't matter but existing series AND volume do.
    /// 6. Fetch series ID(s)
    /// 7. Insert into book and series link table.
    /// 8. Insert into book and author link table.
    /// # Errors
    /// There are two ways in which this function call can fail:
    /// 1. Problem with database
    /// 2. Book already exists in database (only unique books allowed)
    #[allow(
        clippy::missing_inline_in_public_items,
        reason = "Called rarely, large function"
    )]
    pub async fn insert_book(&self, book: &BookRecord) -> Result<(), InsertBookError> {
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
                    return Err(InsertBookError::BookAlreadyExists(book.goodreads_id));
                }
                return Err(InsertBookError::Db(error));
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
