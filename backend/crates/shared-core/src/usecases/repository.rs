use crate::ports::filesystem::{FileSystemError, FileSystemPort};
use std::{path::Path, sync::Arc};

pub struct CreateRepositoryUseCase {
    /// file system adapter
    filesystem: Arc<dyn FileSystemPort + Send + Sync>,
}

impl CreateRepositoryUseCase {
    #[inline]
    pub fn new(filesystem: Arc<dyn FileSystemPort + Send + Sync>) -> Self {
        Self { filesystem }
    }

    /// Perform the task (create a new repository)
    ///
    /// # Errors
    /// This method simply passes on all errors arising from all used ports
    #[inline]
    pub fn execute(&self, path: &Path) -> Result<(), CreateRepositoryError> {
        self.filesystem.create_file(path)?;
        Ok(())
    }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum CreateRepositoryError {
    #[error(transparent)]
    Generic(#[from] FileSystemError),
}
