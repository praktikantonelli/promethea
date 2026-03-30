use shared_core::ports::repository::BookRepositoryPort;
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

    async fn fetch_all_books(
        &self,
    ) -> Result<Vec<shared_core::domain::metadata::BookRecord>, FetchError> {
    }

    async fn try_fetch_author_sort(&self, author_name: &str) -> Result<Option<String>, FetchError> {
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
