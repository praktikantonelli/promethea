use crate::ports::{
    filesystem::{FileSystemError, FileSystemPort},
    repository::BookRepositoryPort,
};
use std::{path::Path, sync::Arc};

pub struct CreateRepositoryUseCase {
    /// file system adapter
    filesystem: Arc<dyn FileSystemPort + Send + Sync>,
}

impl CreateRepositoryUseCase {
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

#[derive(Debug, thiserror::Error)]
pub enum CreateRepositoryError {
    #[error(transparent)]
    Generic(#[from] FileSystemError),
}

pub struct OpenRepositoryUseCase {
    /// file system adapter
    filesystem: Arc<dyn FileSystemPort + Send + Sync>,
    /// repository adapter
    repository: Arc<dyn BookRepositoryPort + Send + Sync>,
}
