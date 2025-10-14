use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite, SqlitePool,
};
use std::sync::Arc;
use tauri::{AppHandle, Manager, State, Wry};
use tauri_plugin_store::StoreExt;
use tokio::sync::RwLock;

static MIGRATIONS: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

struct Db(Arc<RwLock<DbState>>);

#[derive(Default)]
struct DbState {
    pool: Option<Pool<Sqlite>>,
    db_path: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BookRecord {
    book_id: usize,
    title: String,
    sort: String,
    authors: Vec<String>,
    authors_sort: Vec<String>,
    series_and_volume: Vec<SeriesAndVolume>,
    number_of_pages: u32,
    goodreads_id: u64,
    date_added: DateTime<Utc>,
    date_published: DateTime<Utc>,
    date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SeriesAndVolume {
    series: String,
    sort: String,
    volume: f64,
}

fn sqlite_url_from_path(path: &std::path::Path) -> Result<String, anyhow::Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        std::fs::File::create(path)?;
    }
    Ok(format!("sqlite://{}?mode=rwc", path.to_string_lossy()))
}

async fn init_pool_at_path(path: &str) -> Result<Pool<Sqlite>, anyhow::Error> {
    let url = sqlite_url_from_path(std::path::Path::new(path))?;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    let _ = sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await;
    let _ = sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await;

    MIGRATIONS.run(&pool).await?;
    Ok(pool)
}

async fn try_bootstrap_from_store(app: AppHandle, db: Db) -> Result<(), anyhow::Error> {
    let store = app.store("promethea-config.json")?;
    let path_opt = store.get("library_path");

    if let Some(path) = path_opt {
        let path = path.as_str().unwrap().to_owned();
        let pool = init_pool_at_path(&path).await?;
        {
            let mut guard = db.0.write().await;
            guard.pool = Some(pool);
            guard.db_path = Some(path);
        }
    }
    Ok(())
}

#[tauri::command]
async fn get_db_status(db: State<'_, Db>) -> Result<serde_json::Value, String> {
    let guard = db.0.read().await;
    Ok(serde_json::json!({
      "has_path": guard.db_path.is_some(),
      "db_path": guard.db_path,
      "ready": guard.pool.is_some()
    }))
}

#[tauri::command]
async fn set_db_path(app: AppHandle, db: State<'_, Db>, path: String) -> Result<(), Error> {
    // Initialize pool at the given path
    let pool = init_pool_at_path(&path)
        .await
        .map_err(|e| e.to_string())
        .unwrap();

    // Persist to store
    let store = app.store("promethea-config.json")?;
    store.set("library_path".to_string(), serde_json::json!(path));
    store.save().map_err(|e| e.to_string()).unwrap();

    // Swap the state
    {
        let mut guard = db.0.write().await;
        guard.pool = Some(pool);
        guard.db_path = Some(path);
    }

    Ok(())
}

#[tauri::command]
async fn clear_db_path(app: tauri::AppHandle, db: State<'_, Db>) -> Result<(), String> {
    let store = app.store("promethea-config.json").unwrap();
    store.delete("library_path");
    store.save().map_err(|e| e.to_string())?;

    let mut guard = db.0.write().await;
    guard.pool = None;
    guard.db_path = None;
    Ok(())
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to open key-value config")]
    StoreAccess(#[from] tauri_plugin_store::Error),
    #[error("{0}")]
    Other(String),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Other(e.to_string())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    log::warn!("Received request: greet()");
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn notify_library_path_set(app: AppHandle<Wry>) -> Result<(), Error> {
    log::info!("Received request: notify_library_path_set()");
    let store = app.store("promethea-config.json")?;
    let path = store.get("library_path");
    dbg!(path);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init());
    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
    }

    builder
        .setup(|app| {
            // Let app manage SQLite database state
            app.manage(Db(Arc::new(RwLock::new(DbState::default()))));
            let app_handle = app.handle().clone();
            let db_state = app.state::<Db>().inner().0.clone();
            tauri::async_runtime::spawn(async move {
                let _ = try_bootstrap_from_store(app_handle, Db(db_state)).await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            notify_library_path_set,
            get_db_status,
            set_db_path,
            clear_db_path
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
