use crate::errors::Error;
use crate::state::{AppState, APP_CONFIG_PATH, LIBRARY_DATABASE_NAME};
use epub::doc::EpubDoc;
use promethea_core::database::types::BookRecord;
use promethea_core::scraper::request_builder::MetadataRequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DbInitStatus {
    Loaded,
    NeedsSetup { reason: Option<String> },
}

#[tauri::command]
pub async fn create_new_db(
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
pub async fn open_existing_db(
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
pub async fn get_init_status(state: State<'_, AppState>) -> Result<DbInitStatus, ()> {
    if state.db.read().await.is_some() {
        Ok(DbInitStatus::Loaded)
    } else {
        Ok(DbInitStatus::NeedsSetup {
            reason: state.last_error.read().await.clone(),
        })
    }
}

#[tauri::command]
pub async fn fetch_books(state: State<'_, AppState>) -> Result<Vec<BookRecord>, String> {
    let read_guard = state.db.read().await;
    if let Some(db) = &*read_guard {
        let books = db.fetch_books_query();
        return books.await.map_err(|e| format!("Failed to run query: {e}"));
    }

    Err(String::from("Database pool unavailable"))
}

#[tauri::command]
pub async fn add_book(state: State<'_, AppState>, path: PathBuf) -> Result<(), Error> {
    log::info!("Received request to add book from {path:?}");

    // Extract bare minimum metadata (title + author(s)) from EPUB file
    let doc = EpubDoc::new(path).unwrap();
    dbg!(&doc.metadata);

    let title = doc.get_title().unwrap();
    let authors = doc
        .metadata
        .iter()
        .filter(|e| e.property == "creator")
        .map(|e| e.value.clone())
        .collect::<Vec<String>>();

    // Use those title and author(s) to find the appropriate book on Goodreads and scrape it for
    // more data
    let request = MetadataRequestBuilder::default()
        .with_title(&title)
        .with_author(authors.first().unwrap());

    match request.execute().await.unwrap() {
        Some(metadata) => {
            dbg!(metadata);
        }
        None => log::info!("No metadata found for this book"),
    }

    // At this point, we have:
    // Book title and Goodreads ID
    // Author(s) and Goodreads ID(s)
    // Series name, volume and Goodreads ID
    // Page count
    // Publication date
    //
    // MISSING:
    // Title sort string => Titles are generally unique, use sort function
    // Author(s) sort string(s) => In order to handle special cases once, first look if available
    // in database already
    // Series sort string(s) => Same as authors
    // Date added => get today's date
    // Date updated => get today's date

    // Assemble data into SQL query

    // Basic logic: Upsert new book title, author(s) name(s) and series title(s), meaning try to
    // insert and then fetch resulting ID, do not insert if already present and fetch previously
    // existing ID.
    //
    // In SQLite, upsert either with
    //
    // INSERT INTO series (name)
    // VALUES (?)
    // ON CONFLICT(name) DO
    // UPDATE SET name = excluded.name RETURNING id;
    //
    // or
    //
    // INSERT OR IGNORE INTO series (name) VALUES (?);
    // SELECT id FROM series WHERE name = ?;
    //
    // After doing that for books, authors and series, take all IDs and update linking tables. Wrap
    // all queries between one BEGIN; and one COMMIT;

    // For sorting, define helper functions for common stuff like titles starting with "The", "A",
    // "An", and for authors try "Lastname, Firstname"

    Ok(())
}
