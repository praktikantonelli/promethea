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

pub enum BackendState {
    /// backend not ready yet
    NeedsSetup,
    /// backend ready and services up and running
    Ready(ApplicationServices),
}

pub struct ApplicationServices {
    pub add_book: Arc<AddBookUseCase>,
    pub fetch_books: Arc<FetchBooksUseCase>,
}

pub struct RuntimeConfig {
    pub library_path: Option<PathBuf>,
}

#[non_exhaustive]
pub struct AppState {
    pub config: Arc<RwLock<RuntimeConfig>>,
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
