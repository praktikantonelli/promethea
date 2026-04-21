use adapters::{filesystem::FileSystem, metadata::MetadataProvider, repository::Database};
use shared_core::ports::{
    filesystem::FileSystemPort, metadata::MetadataProviderPort, repository::BookRepositoryPort,
};
use shared_core::usecases::books::AddBookUseCase;
use shared_core::usecases::repository::FetchBooksUseCase;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::RwLock;

use crate::errors::PrometheaError;

/// Name of the config file that is stored in the default Tauri config location
pub const APP_CONFIG_PATH: &str = "promethea-config.json";
/// Name of the `SQLite` database file that represents the library
pub const LIBRARY_DATABASE_NAME: &str = "library.db";

/// The state in which the backend is currently in. Starts as `NeedsSetup` and gets set to `Ready`
/// when all services are up and running
pub enum BackendState {
    /// backend not ready yet
    NeedsSetup,
    /// backend ready and services up and running
    Ready(ApplicationServices),
}

/// Collects all use cases of the application
pub struct ApplicationServices {
    /// add a new book to the repository
    pub add_book: Arc<AddBookUseCase>,
    /// fetch all owned books from the repository
    pub fetch_books: Arc<FetchBooksUseCase>,
}

/// Models all runtime data for the application
pub struct RuntimeConfig {
    /// current path where the book library is stored on disk
    pub library_path: Option<PathBuf>,
}

#[non_exhaustive]
/// The state of the app, everything dealing with backend logic
pub struct AppState {
    /// runtime data of the app's state
    pub config: Arc<RwLock<RuntimeConfig>>,
    /// backend state, including whether ready or not
    pub backend: Arc<RwLock<BackendState>>,
}

/// Initializes all use cases and ports/adapters
pub async fn build_services(library_path: PathBuf) -> Result<ApplicationServices, PrometheaError> {
    let repository: Arc<dyn BookRepositoryPort + Send + Sync> =
        Arc::new(Database::open(&library_path).await?);
    let filesystem: Arc<dyn FileSystemPort + Send + Sync> = Arc::new(FileSystem::new());
    let metadata: Arc<dyn MetadataProviderPort + Send + Sync> =
        Arc::new(MetadataProvider::create()?);

    let add_book = Arc::new(AddBookUseCase::new(
        repository.clone(),
        metadata.clone(),
        filesystem.clone(),
    ));
    let fetch_books = Arc::new(FetchBooksUseCase::new(repository.clone()));

    Ok(ApplicationServices {
        add_book,
        fetch_books,
    })
}
