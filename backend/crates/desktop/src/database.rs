use crate::errors::PrometheaError;
use crate::state::{
    APP_CONFIG_PATH, AppState, BackendState, LIBRARY_DATABASE_NAME, build_services,
};
use serde_json::json;
use shared_core::domain::repository::BookItem;
use shared_core::usecases::books::AddBookInput;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter as _, State};
use tauri_plugin_store::StoreExt as _;

#[tauri::command]
/// Open a database connection, creating the database if it doesn't exist yet
pub async fn open_db(
    state: State<'_, AppState>,
    app: AppHandle,
    path_str: String,
) -> Result<(), PrometheaError> {
    let mut path = PathBuf::from(path_str);
    if path.is_dir() {
        // path points to directory where a new file has to be created
        path = path.join(PathBuf::from(LIBRARY_DATABASE_NAME));
    }

    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": path.to_str() }));
    log::info!("Set database path in store to {}", path.display());

    {
        let mut config = state.config.write().await;
        config.library_path = Some(path.clone());
    }

    let services = build_services(path).await?;

    {
        let mut backend = state.backend.write().await;
        *backend = BackendState::Ready(services);
    }

    // if an existing DB was opened, it might already contain values
    app.emit("db:changed", ())?;

    Ok(())
}

/// Fetches the database's initialization status
#[tauri::command]
pub async fn get_init_status(state: State<'_, AppState>) -> Result<bool, PrometheaError> {
    let backend = state.backend.read().await;
    let config = state.config.read().await;
    if config.library_path.is_none() {
        tracing::warn!("Database path not yet configured!");
        Ok(false)
    } else {
        Ok(matches!(&*backend, BackendState::Ready(_)))
    }
}

/// Wrapper around fetch query that returns a vector containing all books and their metadata, to be
/// displayed in the GUI as a list/table/card stack
#[tauri::command]
pub async fn fetch_books(state: State<'_, AppState>) -> Result<Vec<BookItem>, PrometheaError> {
    let use_case = {
        let backend = state.backend.read().await;

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
        let backend = state.backend.read().await;

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
