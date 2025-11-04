use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{sqlite::SqliteConnectOptions, Pool, Row, Sqlite, SqlitePool};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_store::StoreExt;
use tokio::sync::{Mutex, RwLock};

const APP_CONFIG_PATH: &str = "promethea-config.json";
const LIBRARY_DATABASE_NAME: &str = "library.db";

struct AppState {
    db_pool: RwLock<Option<SqlitePool>>,
    failed_to_load_db: RwLock<bool>,
}

impl AppState {
    fn new() -> Self {
        Self {
            db_pool: RwLock::new(None),
            failed_to_load_db: RwLock::new(false),
        }
    }
    async fn init_db_with_path(&self, path: PathBuf) -> anyhow::Result<()> {
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

#[derive(Serialize, Deserialize, Clone)]
pub struct BookRecord {
    book_id: i64,
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
async fn test_database(db_state: State<'_, Mutex<Pool<Sqlite>>>) -> Result<BookRecord, String> {
    let pool = db_state.lock().await;
    let query = "WITH series_info AS (
                SELECT
                    bsl.book,
                    json_group_array(
                        json_object(
                            'series', s.name, 'sort', s.sort, 'volume', bsl.entry
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
                    json_group_array(a.name) authors,
                    json_group_array(a.sort) authors_sort,
                    bal.book
                FROM
                    authors AS a
                    JOIN books_authors_link bal ON a.id = bal.author
                GROUP BY
                    bal.book
            )
            SELECT
                id, title, sort, date_added, date_published, last_modified, number_of_pages, goodreads_id, authors, authors_sort, series_and_volume
            FROM
                books
                LEFT JOIN series_info ON series_info.book = books.id
                JOIN authors_info ON authors_info.book = books.id
            ORDER BY
                books.date_added ASC";
    match sqlx::query(query).fetch_one(&*pool).await {
        Ok(row) => Ok(BookRecord {
            book_id: row.get("id"),
            title: row.get("title"),
            sort: row.get("sort"),
            authors: serde_json::from_str(row.get("authors")).unwrap(),
            authors_sort: serde_json::from_str(row.get("authors_sort")).unwrap(),
            series_and_volume: serde_json::from_str(row.get("series_and_volume")).unwrap(),
            number_of_pages: row.get("number_of_pages"),
            goodreads_id: row.get("goodreads_id"),
            date_added: row.get("date_added"),
            date_published: row.get("date_published"),
            date_modified: row.get("date_modified"),
        }),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn create_new_db(app: AppHandle, folder: String) -> Result<(), Error> {
    let db_file_path = PathBuf::from(folder).join(PathBuf::from(LIBRARY_DATABASE_NAME));
    std::fs::File::create(db_file_path.clone()).unwrap();

    // update config store
    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!("Updated database path in store to {db_file_path:?}");

    Ok(())
}

#[tauri::command]
fn open_existing_db(app: AppHandle, path: String) -> Result<(), Error> {
    let db_file_path = PathBuf::from(path);

    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!("Updated database path in store to {db_file_path:?}");

    Ok(())
}

#[tauri::command]
async fn database_loading_failed(state: State<'_, Mutex<AppState>>) -> Result<bool, Error> {
    let state = state.lock().await;
    let flag = *state.failed_to_load_db.read().await;
    Ok(flag)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init());
    builder
        .manage(AppState::new())
        .setup(|app| {
            // Let app manage SQLite database state
            let (tauri_plugin_log, max_level, logger) = tauri_plugin_log::Builder::default()
                .with_colors(ColoredLevelConfig::default())
                .split(app.handle())?;

            if cfg!(debug_assertions) {
                // With debug assertions, use CrabNebula dev tools plugin
                let mut devtools_builder = tauri_plugin_devtools::Builder::default();
                devtools_builder.attach_logger(logger);
                app.handle().plugin(devtools_builder.init())?;
            } else {
                // Without debug assertions, use regular logger plugin
                tauri_plugin_log::attach_logger(max_level, logger)?;
            }
            app.handle().plugin(tauri_plugin_log)?;

            let store = app.store(APP_CONFIG_PATH).unwrap();
            if let Some(db_path) = store.get("library-path") {
                log::info!("Using database at {db_path:?}");
                let app_state = app.state::<AppState>().clone();
                tauri::async_runtime::block_on(async move {
                    let path = PathBuf::from(db_path.get("value").unwrap().as_str().unwrap());
                    if let Err(err) = app_state.init_db_with_path(path).await {
                        log::error!("DB init on startup failed: {err}");
                        let mut db_loading_failed_guard = app_state.failed_to_load_db.write().await;
                        *db_loading_failed_guard = true;
                    } else {
                        log::info!("DB connected successfully");
                    }
                })
            } else {
                log::info!("No database path in config, wait for user to provide one");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_new_db,
            open_existing_db,
            database_loading_failed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
