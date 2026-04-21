use crate::errors::PrometheaError;
use crate::state::{
    APP_CONFIG_PATH, AppState, BackendState, LIBRARY_DATABASE_NAME, build_services,
};
use serde_json::json;
use shared_core::domain::repository::BookItem;
use shared_core::usecases::books::AddBookInput;
use std::fs::File;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter as _, State};
use tauri_plugin_store::StoreExt as _;

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
        let mut config = state
            .config
            .write()
            .map_err(|error| PrometheaError::State {
                message: error.to_string(),
            })?;
        config.library_path = Some(db_file_path.clone());
    }

    let services = build_services(db_file_path).await?;

    {
        let mut backend = state
            .backend
            .write()
            .map_err(|error| PrometheaError::State {
                message: error.to_string(),
            })?;
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
        let mut config = state
            .config
            .write()
            .map_err(|error| PrometheaError::State {
                message: error.to_string(),
            })?;
        config.library_path = Some(db_file_path.clone());
    }

    let services = build_services(db_file_path).await?;

    {
        let mut backend = state
            .backend
            .write()
            .map_err(|error| PrometheaError::State {
                message: error.to_string(),
            })?;
        *backend = BackendState::Ready(services);
    }

    Ok(())
}

/// Fetches the database's initialization status
#[tauri::command]
pub async fn get_init_status(state: State<'_, AppState>) -> Result<bool, PrometheaError> {
    let config = state.config.read().map_err(|error| PrometheaError::State {
        message: error.to_string(),
    })?;
    Ok(config.library_path.is_some())
}

/// Wrapper around fetch query that returns a vector containing all books and their metadata, to be
/// displayed in the GUI as a list/table/card stack
#[tauri::command]
pub async fn fetch_books(state: State<'_, AppState>) -> Result<Vec<BookItem>, PrometheaError> {
    let use_case = {
        let backend = state
            .backend
            .read()
            .map_err(|error| PrometheaError::State {
                message: error.to_string(),
            })?;

        match &*backend {
            BackendState::NeedsSetup => {
                return Err(PrometheaError::State {
                    message: "Backend not set up yet!".to_owned(),
                });
            }
            BackendState::Ready(services) => services.fetch_books.clone(),
        }
    };
    let output = use_case
        .execute()
        .await
        .map_err(|error| PrometheaError::Other(error.to_string()))?;
    Ok(output.0)
}

/// Given a path to an EPUB file, extracts title and author(s) and uses that to fetch metadata,
/// then inserts all data into the database
#[tauri::command]
pub async fn add_book(
    app: AppHandle,
    state: State<'_, AppState>,
    path: PathBuf,
) -> Result<(), PrometheaError> {
    tracing::info!("Received request to add book from {path:?}");
    let use_case = {
        let backend = state
            .backend
            .read()
            .map_err(|error| PrometheaError::State {
                message: error.to_string(),
            })?;

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
