use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use tokio::sync::RwLock;

pub const APP_CONFIG_PATH: &str = "promethea-config.json";
pub const LIBRARY_DATABASE_NAME: &str = "library.db";

pub struct AppState {
    pub db_pool: RwLock<Option<SqlitePool>>,
    pub last_error: RwLock<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db_pool: RwLock::new(None),
            last_error: RwLock::new(None),
        }
    }
    pub async fn connect_db_with_path(&self, path: PathBuf) -> anyhow::Result<()> {
        log::info!("Creating SQLite pool for DB at {path:?}");
        let options = SqliteConnectOptions::new()
            .foreign_keys(true)
            .filename(path.clone());
        let pool = SqlitePool::connect_with(options).await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        log::info!("Successfully opened database at {path:?}");

        let mut guard = self.db_pool.write().await;
        // guard.replace(pool) puts pool into Option<SqlitePool> and returns the contained value if
        // there was one
        if let Some(old) = guard.replace(pool) {
            // if Option<SqlitePool> had value, close pool
            log::info!("Found old SQLite pool in AppDb state, closing...");
            old.close().await;
        }

        Ok(())
    }
}
