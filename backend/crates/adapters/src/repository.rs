use shared_core::{domain::metadata::BookRecord, ports::repository::BookRepositoryPort};
use sqlx::{Sqlite, SqlitePool, sqlite::SqliteConnectOptions};

pub struct DataBase {
    pool: SqlitePool,
}

impl BookRepositoryPort for DataBase {
    async fn open(path: &std::path::Path) -> Result<Self, OpenRepositoryError> {
        let options = SqliteConnectOptions::new()
            .foreign_keys(true)
            .filename(path);
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

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
        .await?;
        Ok(books)
    }

    async fn try_fetch_author_sort(&self, author_name: &str) -> Result<Option<String>, FetchError> {
        let sort = sqlx::query!("SELECT sort FROM authors WHERE name LIKE ?", author_name)
            .fetch_one(&self.pool)
            .await?
            .sort;

        Ok(Some(sort))
    }

    async fn try_fetch_series_sort(
        &self,
        series_title: &str,
    ) -> Result<Option<String>, FetchError> {
    }

    async fn insert_book(
        &self,
        book: shared_core::domain::metadata::BookRecord,
    ) -> Result<(), InsertBookError> {
    }

    async fn update_book(
        &self,
        book: shared_core::domain::metadata::BookRecord,
    ) -> Result<(), UpdateError> {
    }

    async fn update_series(
        &self,
        series: shared_core::domain::metadata::SeriesAndVolumeRecord,
    ) -> Result<(), UpdateError> {
    }

    async fn update_author(
        &self,
        author: shared_core::domain::metadata::AuthorRecord,
    ) -> Result<(), UpdateError> {
    }
}
