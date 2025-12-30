use crate::database::types::BookRecord;
use sqlx::{Row, SqlitePool, sqlite::SqliteConnectOptions};
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
        let query = "
            WITH series_info AS (
                SELECT 
                    bsl.book, 
                    Json_group_array(
                        Json_object(
                            'series', s.NAME, 'sort', s.sort, 'volume', 
                            bsl.entry
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
                    Json_group_array(a.NAME) authors, 
                    Json_group_array(a.sort) authors_sort, 
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
                authors_sort, 
                CASE WHEN series_and_volume IS NULL 
                OR Trim(series_and_volume) = '' THEN '[]' WHEN Json_valid
                    (series_and_volume) = 1 THEN series_and_volume ELSE '[]' END AS 
                    series_and_volume 
            FROM 
                books 
                LEFT JOIN series_info ON series_info.book = books.id 
                JOIN authors_info ON authors_info.book = books.id 
            ORDER BY 
                books.date_added ASC;";
        let books: Vec<BookRecord> = sqlx::query_as(query).fetch_all(&self.pool).await?;
        Ok(books)
    }

    pub async fn try_fetch_author_sort(&self, name: &str) -> Result<Option<String>, sqlx::Error> {
        let query = "
            SELECT sort FROM authors WHERE name LIKE $1;
        ";
        let sort = sqlx::query(query)
            .bind(name)
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(Some(sort))
    }

    pub async fn try_fetch_series_sort(&self, name: &str) -> Result<Option<String>, sqlx::Error> {
        let query = "
            SELECT sort FROM series WHERE name LIKE $1;
        ";
        let sort = sqlx::query(query)
            .bind(name)
            .fetch_one(&self.pool)
            .await?
            .get(0);

        Ok(Some(sort))
    }

    pub async fn insert_book(&self, book: BookRecord) -> Result<(), sqlx::Error> {
        // Query outline:
        // 1. Insert book (title, sort, date_added, date_published, last_modified, number_of_pages, goodreads_id)
        // 2. Fetch book ID (either newly created through operation 1 or already there and retrieved)
        // 3. Insert author(s) (name, sort, goodreads_id)
        // 4. Fetch author IDs (same principle as book ID)
        // 5. Insert series (name, sort, volume, goodreads_id)
        // 6. Fetch series IDs (same principle as books and authors)
        // 7. Insert book series link (book ID, series ID(s))
        // 8. Insert book authors link (book ID, author(s) ID(s))
        let query = "
            BEGIN;
            

            END;
        ";

        Ok(())
    }
}
