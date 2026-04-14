use async_trait::async_trait;
use std::path::Path;

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
#[async_trait]
pub trait FileSystemPort {
    async fn create_file(&self, path: &Path) -> Result<(), FileSystemError>;

    async fn move_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    async fn copy_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    async fn delete_file(&self, path: &Path) -> Result<(), FileSystemError>;

    /// Extract the title of a book from an EPUB file
    ///
    /// # Errors
    /// Returns an error if there is a problem with the file system, or if the title cannot be
    /// extracted (e.g., because the file doesn't contain a title field).
    fn extract_title_from_epub(&self, path: &Path) -> Result<String, FileSystemError>;

    /// Extract the author of a book from an EPUB file
    ///
    /// # Errors
    /// Returns an error if there is a problem with the file system, or if the author cannot be
    /// extracted (e.g., because the file doesn't contain an author field).
    fn extract_author_from_epub(&self, path: &Path) -> Result<String, FileSystemError>;
}

/// Error with the file system
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum FileSystemError {
    #[error("error in file system: `{message}`")]
    Generic { message: String },
    #[error("value not found: `{message}`")]
    Value { message: String },
}
