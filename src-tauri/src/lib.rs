use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_store::StoreExt;
use tokio::sync::RwLock;

const APP_CONFIG_PATH: &str = "promethea-config.json";
const LIBRARY_DATABASE_NAME: &str = "library.db";

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum DbInitStatus {
    Loaded,
    NeedsSetup { reason: Option<String> },
}

struct AppState {
    db_pool: RwLock<Option<SqlitePool>>,
    last_error: RwLock<Option<String>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            db_pool: RwLock::new(None),
            last_error: RwLock::new(None),
        }
    }
    async fn connect_db_with_path(&self, path: PathBuf) -> anyhow::Result<()> {
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

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct BookRecord {
    book_id: i64,
    title: String,
    sort: String,
    #[sqlx(json)]
    authors: Vec<String>,
    #[sqlx(json)]
    authors_sort: Vec<String>,
    #[sqlx(json)]
    series_and_volume: Vec<SeriesAndVolume>,
    number_of_pages: u32,
    goodreads_id: u64,
    date_added: DateTime<Utc>,
    date_published: DateTime<Utc>,
    date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
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
async fn fetch_books(state: State<'_, AppState>) -> Result<Vec<BookRecord>, String> {
    let read_guard = state.db_pool.read().await;
    if let Some(pool) = &*read_guard {
        let query = "
            WITH series_info
                AS (SELECT bsl.book,
                            Json_group_array(Json_object('series', s.NAME, 'sort', s.sort,
                                            'volume',
                                            bsl.entry)) series_and_volume
                    FROM   series AS s
                            JOIN books_series_link bsl
                            ON bsl.series = s.id
                    GROUP  BY bsl.book),
                authors_info
                AS (SELECT Json_group_array(a.NAME) authors,
                            Json_group_array(a.sort) authors_sort,
                            bal.book
                    FROM   authors AS a
                            JOIN books_authors_link bal
                            ON a.id = bal.author
                    GROUP  BY bal.book)
            SELECT id            AS book_id,
                title,
                sort,
                date_added,
                date_published,
                last_modified AS date_modified,
                number_of_pages,
                goodreads_id,
                authors,
                authors_sort,
                CASE
                    WHEN series_and_volume IS NULL
                        OR Trim(series_and_volume) = '' THEN '[]'
                    WHEN Json_valid(series_and_volume) = 1 THEN series_and_volume
                    ELSE '[]'
                END           AS series_and_volume
            FROM   books
                LEFT JOIN series_info
                        ON series_info.book = books.id
                JOIN authors_info
                    ON authors_info.book = books.id
            ORDER  BY books.date_added ASC  ";
        let books: Vec<BookRecord> = sqlx::query_as(query).fetch_all(pool).await.unwrap();
        return Ok(books);
    }

    Err(String::from("Database pool unavailable"))
}

#[tauri::command]
async fn create_new_db(
    state: State<'_, AppState>,
    app: AppHandle,
    folder: String,
) -> Result<(), Error> {
    let db_file_path = PathBuf::from(folder).join(PathBuf::from(LIBRARY_DATABASE_NAME));
    std::fs::File::create(db_file_path.clone()).unwrap();

    // update config store
    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!("Updated database path in store to {db_file_path:?}");

    state.connect_db_with_path(db_file_path).await?;

    Ok(())
}

#[tauri::command]
async fn open_existing_db(
    state: State<'_, AppState>,
    app: AppHandle,
    path: String,
) -> Result<(), Error> {
    let db_file_path = PathBuf::from(path);

    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!("Updated database path in store to {db_file_path:?}");

    state.connect_db_with_path(db_file_path).await?;

    Ok(())
}

#[tauri::command]
async fn get_init_status(state: State<'_, AppState>) -> Result<DbInitStatus, ()> {
    if state.db_pool.read().await.is_some() {
        Ok(DbInitStatus::Loaded)
    } else {
        Ok(DbInitStatus::NeedsSetup {
            reason: state.last_error.read().await.clone(),
        })
    }
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
                    if let Err(err) = app_state.connect_db_with_path(path).await {
                        log::error!("DB init on startup failed: {err}");
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
            get_init_status,
            fetch_books
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
