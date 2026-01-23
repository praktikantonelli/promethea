use promethea_core::database::queries::Db;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// Name of the config file that is stored in the default Tauri config location
/// Windows: C:\Users\${user}\AppData\Roaming\com.lucaa.promethea\promethea-config.json
pub const APP_CONFIG_PATH: &str = "promethea-config.json";
/// Name of the `SQLite` database file that represents the library
pub const LIBRARY_DATABASE_NAME: &str = "library.db";

pub struct AppState {
    pub db: RwLock<Option<Db>>,
    pub last_error: RwLock<Option<String>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: RwLock::new(None),
            last_error: RwLock::new(None),
        }
    }
    pub async fn connect_db_with_path(&self, path: PathBuf) -> anyhow::Result<()> {
        log::info!("Creating SQLite pool for DB at {path:?}");
        let db = Db::init(&path).await?;
        log::info!("Successfully opened database at {path:?}");

        let mut guard = self.db.write().await;
        // guard.replace(pool) puts pool into Option<SqlitePool> and returns the contained value if
        // there was one
        if let Some(old) = guard.replace(db) {
            // if Option<SqlitePool> had value, close pool
            log::info!("Found old SQLite pool in AppDb state, closing...");
            old.close().await;
        }

        Ok(())
    }
}
