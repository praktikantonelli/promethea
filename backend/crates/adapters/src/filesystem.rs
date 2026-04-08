use shared_core::ports::filesystem::{FileSystemError, FileSystemPort};
use std::path::Path;

pub struct FileSystem {}

impl FileSystemPort for FileSystem {
    async fn create_file(&self, path: &Path) -> Result<(), FileSystemError> {}

    async fn move_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError> {}

    async fn copy_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError> {}

    async fn delete_file(&self, path: &Path) -> Result<(), FileSystemError> {}
}
