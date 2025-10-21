use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const APP_CONFIG_PATH: &str = "promethea-config.json";
const LIBRARY_DATABASE_NAME: &str = "library.db";

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
fn create_new_db(app: AppHandle, folder: String) -> Result<(), Error> {
    let db_file_path = PathBuf::from(folder).join(PathBuf::from(LIBRARY_DATABASE_NAME));
    std::fs::File::create(db_file_path.clone()).unwrap();

    // update config store
    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));

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
            let app_handle = app.handle().clone();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![create_new_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
