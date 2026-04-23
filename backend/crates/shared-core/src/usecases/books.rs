use crate::ports::{
    filesystem::{FileSystemError, FileSystemPort},
    metadata::{FetchMetadataError, MetadataProviderPort},
    repository::{BookRepositoryPort, InsertError},
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct AddBookUseCase {
    /// book repository adapter
    repository: Arc<dyn BookRepositoryPort + Send + Sync>,
    /// metadata fetching adapter
    metadata: Arc<dyn MetadataProviderPort + Send + Sync>,
    /// file system adapter
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
        let path = input.input_path;
        let title = self.filesystem.extract_title_from_epub(&path)?;
        let author = self.filesystem.extract_author_from_epub(&path)?;
        let goodreads_id_opt = self.metadata.fetch_goodreads_id(&title, &author).await?;
        if let Some(goodreads_id) = goodreads_id_opt {
            // fetch metadata
            let metadata = self.metadata.fetch_metadata(goodreads_id).await?;
            // insert metadata into DB
            self.repository.insert_book(metadata).await?;
            // move file to proper location in library folder
            Ok(AddBookOutput)
        } else {
            Err(AddBookError::Metadata(FetchMetadataError::GoodreadsId {
                title,
                author,
            }))
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum AddBookError {
    #[error("failed to extract value from EPUB file")]
    FileSystem(#[from] FileSystemError),
    #[error("failed to fetch metadata")]
    Metadata(#[from] FetchMetadataError),
    #[error("failed to insert book into repository")]
    RepositoryError(#[from] InsertError),
}

#[non_exhaustive]
pub struct AddBookInput {
    /// Path to EPUB file that should be added
    input_path: PathBuf,
}

impl AddBookInput {
    #[inline]
    #[must_use]
    pub fn new(input_path: &Path) -> Self {
        Self {
            input_path: input_path.to_owned(),
        }
    }
}

#[non_exhaustive]
pub struct AddBookOutput;
