use std::path::{Path, PathBuf};

pub trait FileSystemPort {
    /// Create a file somewhere on the system
    ///
    /// # Errors
    /// Returns an error if the file already exists or if there are permission errors
    fn create_file(&self, path: &Path) -> Result<(), FileSystemError>;

    /// Move/rename a file somewhere on the system
    ///
    /// # Errors
    /// Returns an error if the file types don't match (e.g., one path is a file, one a directory)
    /// or if there are permission errors
    fn move_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    /// Copy a file
    ///
    /// # Errors
    /// Returns an error if the file types don't match (e.g., one path is a file, one a directory)
    /// or if there are permission errors
    fn copy_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    /// Delete a file
    ///
    /// # Errors
    /// Returns an error if the file cannot be deleted, e.g., due to permission errors or because
    /// the file doesn't exist
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
