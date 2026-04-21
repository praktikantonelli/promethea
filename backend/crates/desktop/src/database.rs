use crate::errors::PrometheaError;
use crate::new_state::{
    APP_CONFIG_PATH, AppState, BackendState, LIBRARY_DATABASE_NAME, build_services,
};
use chrono::{DateTime, Local};
use core::future::Future;
use core::iter::zip;
use epub::doc::EpubDoc;
use futures::future::join_all;
use promethea_core::database::types::{AuthorRecord, BookRecord, SeriesAndVolumeRecord};
use promethea_core::scraping::sorting::{get_name_sort, get_title_sort};
use serde_json::json;
use shared_core::domain::repository::BookItem;
use std::fs::File;
use std::path::PathBuf;
use std::time::Instant;
use tauri::{AppHandle, Emitter as _, State};
use tauri_plugin_store::StoreExt as _;
use tokio::task;
use tracing::{Instrument as _, info_span, instrument};

/// Wrapper that tries to fetch a value with an async fn and uses a synchronous fallback in case
/// the first operation fails
async fn resolve_sort_with_fallback<Primary, PrimaryFut, Fallback, E>(
    primary: Primary,
    fallback: Fallback,
) -> String
where
    Primary: FnOnce() -> PrimaryFut,
    PrimaryFut: Future<Output = Result<Option<String>, E>>,
    Fallback: FnOnce() -> String,
{
    match primary().await {
        Ok(Some(val)) => val,
        _ => fallback(),
    }
}

/// Creates a new `SQLite` database at a given path writes the path into the Tauri store
#[tauri::command]
pub async fn create_new_db(
    state: State<'_, AppState>,
    app: AppHandle,
    folder: String,
) -> Result<(), PrometheaError> {
    let db_file_path = PathBuf::from(folder).join(PathBuf::from(LIBRARY_DATABASE_NAME));
    File::create(db_file_path.clone()).map_err(|error| {
        PrometheaError::Other(format!("Failed to create database file: {error}"))
    })?;

    // update config store
    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!(
        "Updated database path in store to {}",
        db_file_path.display()
    );

    {
        let mut config = state.config.write().unwrap();
        config.library_path = Some(db_file_path.clone());
    }

    let services = build_services(db_file_path).await?;

    {
        let mut backend = state.backend.write().unwrap();
        *backend = BackendState::Ready(services);
    }

    Ok(())
}

/// Opens an existing database at the given path and updates the stored value in the Tauri store
#[tauri::command]
pub async fn open_existing_db(
    state: State<'_, AppState>,
    app: AppHandle,
    path: String,
) -> Result<(), PrometheaError> {
    let db_file_path = PathBuf::from(path);

    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!(
        "Updated database path in store to {}",
        db_file_path.display()
    );

    {
        let mut config = state.config.write().unwrap();
        config.library_path = Some(db_file_path.clone());
    }

    let services = build_services(db_file_path).await?;

    {
        let mut backend = state.backend.write().unwrap();
        *backend = BackendState::Ready(services);
    }

    Ok(())
}

/// Fetches the database's initialization status
#[tauri::command]
pub async fn get_init_status(state: State<'_, AppState>) -> Result<bool, ()> {
    let config = state.config.read().unwrap();
    if config.library_path.is_some() {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Wrapper around fetch query that returns a vector containing all books and their metadata, to be
/// displayed in the GUI as a list/table/card stack
#[tauri::command]
pub async fn fetch_books(state: State<'_, AppState>) -> Result<Vec<BookItem>, String> {
    let use_case = {
        let backend = state.backend.read().unwrap();

        match &*backend {
            BackendState::NeedsSetup => return Err("not configured".into()),
            BackendState::Ready(services) => services.fetch_books.clone(),
        }
    };
    let output = use_case
        .execute()
        .await
        .map_err(|error| error.to_string())?;
    Ok(output.0)
}

/// Given a path to an EPUB file, extracts title and author(s) and uses that to fetch metadata,
/// then inserts all data into the database
#[allow(
    clippy::significant_drop_tightening,
    reason = "Problem with references due to multiple queries with single Db instance"
)]
#[allow(
    clippy::too_many_lines,
    reason = "Okay for now, will be refactored later"
)]
#[instrument(
    name = "cmd.add_book",
    skip(app, state),
    fields(path = ?path)
)]
#[tauri::command]
pub async fn add_book(
    app: AppHandle,
    state: State<'_, AppState>,
    path: PathBuf,
) -> Result<(), PrometheaError> {
    tracing::info!("Received request to add book from {path:?}");
    let use_case = {
        let backend = state.backend.read().unwrap();

        match &*backend {
            BackendState::NeedsSetup => {
                return Err(PrometheaError::Other("State not ready".to_owned()));
            }
            BackendState::Ready(services) => services.add_book.clone(),
        }
    };
    use_case
        .execute(AddBookInput::new(&path))
        .await
        .map_err(|error| PrometheaError::Other(error.to_string()))?;

    app.emit("db:changed", ())?;
    Ok(())
}
