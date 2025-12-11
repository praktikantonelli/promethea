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
        sqlx::migrate!("src\\database\\migrations")
            .run(&pool)
            .await?;

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
        let query = "
            BEGIN;
            

            END;
        ";

        Ok(())
    }
}
