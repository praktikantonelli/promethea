use crate::ports::{
    filesystem::FileSystemPort, metadata::MetadataProviderPort, repository::BookRepositoryPort,
};
use std::{path::PathBuf, sync::Arc};

pub struct AddBookUseCase {
    repository: Arc<dyn BookRepositoryPort + Send + Sync>,
    metadata: Arc<dyn MetadataProviderPort + Send + Sync>,
    filesystem: Arc<dyn FileSystemPort + Send + Sync>,
}

impl AddBookUseCase {
    #[inline]
    pub fn new(
        repository: Arc<dyn BookRepositoryPort + Send + Sync>,
        metadata: Arc<dyn MetadataProviderPort + Send + Sync>,
        filesystem: Arc<dyn FileSystemPort + Send + Sync>,
    ) -> Self {
        Self {
            repository,
            metadata,
            filesystem,
        }
    }
    /// Adds a book to the library, including updating the DB and the file system
    ///
    /// # Errors
    /// Errors when the book cannot be added for multiple reasons, including
    /// - duplicate book
    /// - error communicating with DB
    /// - error fetching metadata
    /// - error with file system
    #[inline]
    pub async fn execute(&self, input: AddBookInput) -> Result<AddBookOutput, AddBookError> {
        // input: probably path to EPUB file or file itself
        // extract title and author(s)
        // fetch metadata
        // insert metadata into DB
        // move file to proper location in library folder
        Ok(AddBookOutput {})
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum AddBookError {}

#[non_exhaustive]
pub struct AddBookInput {
    /// Path to EPUB file that should be added
    input_path: PathBuf,
}

#[non_exhaustive]
pub struct AddBookOutput {}
