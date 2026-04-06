use crate::ports::{metadata::MetadataProviderPort, repository::BookRepositoryPort};
use std::sync::Arc;

pub struct AddBookUseCase {
    repository: Arc<dyn BookRepositoryPort>,
    metadata: Arc<dyn MetadataProviderPort>,
}

impl AddBookUseCase {
    /// Adds a book to the library, including updating the DB and the file system
    ///
    /// # Errors
    /// Errors when the book cannot be added for multiple reasons, including
    /// - duplicate book
    /// - error communicating with DB
    /// - error fetching metadata
    /// - error with file system
    pub async fn execute(&self) -> Result<(), AddBookError> {
        // input: probably path to EPUB file or file itself
        // extract title and author(s)
        // fetch metadata
        // insert metadata into DB
        // move file to proper location in library folder
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum AddBookError {}
