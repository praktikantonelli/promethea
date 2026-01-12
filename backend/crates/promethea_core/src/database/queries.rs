use crate::database::types::{BookRecord, InsertBookError};
use sqlx::{Row, Sqlite, SqlitePool, Transaction, sqlite::SqliteConnectOptions};
use std::path::Path;

pub struct Db {
    pool: SqlitePool,
}

impl Db {
    pub async fn init(path: &Path) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::new()
            .foreign_keys(true)
            .filename(path);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }

    pub async fn fetch_books_query(&self) -> Result<Vec<BookRecord>, sqlx::Error> {
        let books: Vec<BookRecord> = sqlx::query_as!(
            BookRecord,
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
        "
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(books)
    }

    pub async fn try_fetch_author_sort(&self, name: &str) -> Result<Option<String>, sqlx::Error> {
        let sort = sqlx::query!("SELECT sort FROM authors WHERE name LIKE ?", name)
            .fetch_one(&self.pool)
            .await?
            .sort;

        Ok(Some(sort))
    }

    pub async fn try_fetch_series_sort(&self, name: &str) -> Result<Option<String>, sqlx::Error> {
        let sort = sqlx::query!("SELECT sort FROM series WHERE name LIKE ?", name)
            .fetch_one(&self.pool)
            .await?
            .sort;

        Ok(Some(sort))
    }

    pub async fn insert_book(&self, book: &BookRecord) -> Result<(), InsertBookError> {
        // Query outline:
        // 1. Insert book (title, sort, date_added, date_published, last_modified, number_of_pages, goodreads_id)
        // 2. Fetch book ID (either newly created through operation 1 or already there and retrieved)
        // 3. Insert author(s) (name, sort, goodreads_id)
        // 4. Fetch author IDs (same principle as book ID)
        // 5. Insert series (name, sort, volume, goodreads_id)
        // 6. Fetch series IDs (same principle as books and authors)
        // 7. Insert book series link (book ID, series ID(s))
        // 8. Insert book authors link (book ID, author(s) ID(s))
        let mut tx: Transaction<'_, Sqlite> = self.pool.begin().await?;

        let book_goodreads_id = book.goodreads_id as i64;
        let number_of_pages = book.number_of_pages as i64;

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
            Err(e) => {
                if is_sqlite_unique_violation(&e) {
                    tx.rollback().await.ok();
                    return Err(InsertBookError::BookAlreadyExists(book.goodreads_id));
                }
                return Err(InsertBookError::Db(e));
            }
        };

        // handle authors
        for a in &book.authors {
            let author_goodreads_id = a.goodreads_id as i64;
            let author_id: i64 = sqlx::query!(
                r#"
                    INSERT INTO authors(name, sort, goodreads_id)
                    VALUES (?, ?, ?)
                    ON CONFLICT(goodreads_id) DO UPDATE SET
                        name = excluded.name,
                        sort = excluded.sort
                    RETURNING id;
                "#,
                a.name,
                a.sort,
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
            let sav_goodreads_id = sav.goodreads_id as i64;
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

fn is_sqlite_unique_violation(e: &sqlx::Error) -> bool {
    // Check for unique violation by searching for matching text in error message
    match e {
        sqlx::Error::Database(db_err) => db_err.message().contains("UNIQUE constraint failed"),
        _ => false,
    }
}
