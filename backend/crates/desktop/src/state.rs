use promethea_core::database::queries::Db;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// Name of the config file that is stored in the default Tauri config location
/// Windows: C:\Users\${user}\AppData\Roaming\com.lucaa.promethea\promethea-config.json
pub const APP_CONFIG_PATH: &str = "promethea-config.json";
/// Name of the `SQLite` database file that represents the library
pub const LIBRARY_DATABASE_NAME: &str = "library.db";

/// App state holding the database connection and the last error state
pub struct AppState {
    /// Used for all database queries, which are implemented on `Db`
    pub db: RwLock<Option<Db>>,
    /// Holds the last error state arising from accessing the database
    pub last_error: RwLock<Option<String>>,
}

impl AppState {
    /// Standard constructor
    pub fn new() -> Self {
        Self {
            db: RwLock::new(None),
            last_error: RwLock::new(None),
        }
    }

    /// Takes an existing `AppState` instance and hooks it up with the provided path
    pub async fn connect_db_with_path(&self, path: PathBuf) -> anyhow::Result<()> {
        log::info!("Creating SQLite pool for DB at {}", path.display());
        let db = Db::init(&path).await?;
        log::info!("Successfully opened database at {}", path.display());

        let old = self.db.write().await.replace(db);
        match old {
            Some(old_db) => {
                log::info!("Found old SQLite pool in AppDb state, closing...");
                old_db.close().await;
            }
            None => {
                log::info!("No old SQLite pool in AppDb, nothing to do.");
            }
        }
        Ok(())
    }
}
