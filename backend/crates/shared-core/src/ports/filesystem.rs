use std::path::Path;

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
pub trait FileSystemPort {
    async fn create_file(&self, path: &Path) -> Result<(), FileSystemError>;

    async fn move_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    async fn copy_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError>;

    async fn delete_file(&self, path: &Path) -> Result<(), FileSystemError>;

    fn extract_title_from_epub(&self, path: &Path) -> Result<String, FileSystemError>;

    fn extract_author_from_epub(&self, path: &Path) -> Result<String, FileSystemError>;
}

/// Error with the file system
#[derive(thiserror::Error, Debug)]
enum FileSystemError {}
