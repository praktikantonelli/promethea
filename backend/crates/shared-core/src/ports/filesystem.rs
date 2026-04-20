use std::path::{Path, PathBuf};

pub trait FileSystemPort {
    fn create_file(&self, path: &Path) -> Result<(), FileSystemError>;

    fn move_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    fn copy_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    fn delete_file(&self, path: &Path) -> Result<(), FileSystemError>;

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
    #[error("could not create file at `{path}`: `{message}`")]
    Creation { path: PathBuf, message: String },
    #[error("could not move file from `{source_path}` to `{target_path}`: `{message}`")]
    Move {
        source_path: PathBuf,
        target_path: PathBuf,
        message: String,
    },
    #[error("could not copy file from `{source_path}` to `{target_path}`: `{message}`")]
    Copy {
        source_path: PathBuf,
        target_path: PathBuf,
        message: String,
    },
    #[error("could not delete file at `{path}`: `{message}`")]
    Delete { path: PathBuf, message: String },
}
