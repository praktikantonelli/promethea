use async_trait::async_trait;
use epub::doc::EpubDoc;
use shared_core::ports::filesystem::{FileSystemError, FileSystemPort};
use std::fs;
use std::path::{Path, PathBuf};

#[non_exhaustive]
pub struct FileSystem;

#[async_trait]
impl FileSystemPort for FileSystem {
    async fn create_file(&self, path: &Path) -> Result<(), FileSystemError> {
        fs::File::create_new(path).map_err(|error| FileSystemError::Creation {
            path: PathBuf::from(path),
            message: error.to_string(),
        })?;
        Ok(())
    }

    async fn move_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError> {
        fs::rename(source, target).map_err(|error| FileSystemError::Move {
            source_path: PathBuf::from(source),
            target_path: PathBuf::from(target),
            message: error.to_string(),
        })?;
        Ok(())
    }

    async fn copy_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError> {
        fs::copy(source, target).map_err(|error| FileSystemError::Copy {
            source_path: PathBuf::from(source),
            target_path: PathBuf::from(target),
            message: error.to_string(),
        })?;
        Ok(())
    }

    async fn delete_file(&self, path: &Path) -> Result<(), FileSystemError> {
        fs::remove_file(path).map_err(|error| FileSystemError::Delete {
            path: PathBuf::from(path),
            message: error.to_string(),
        })?;
        Ok(())
    }

    fn extract_title_from_epub(&self, path: &Path) -> Result<String, FileSystemError> {
        EpubDoc::new(path)
            .map_err(|error| FileSystemError::Generic {
                message: error.to_string(),
            })?
            .get_title()
            .ok_or(FileSystemError::Value {
                message: format!("Could not extract title from {path:?}"),
            })
    }

    fn extract_author_from_epub(&self, path: &Path) -> Result<String, FileSystemError> {
        let doc = EpubDoc::new(path).map_err(|error| FileSystemError::Generic {
            message: error.to_string(),
        })?;
        let authors = doc
            .metadata
            .iter()
            .filter(|item| item.property == "creator")
            .map(|item| item.value.clone())
            .collect::<Vec<String>>();
        authors
            .first()
            .ok_or(FileSystemError::Value {
                message: format!("Could not extract author from {path:?}"),
            })
            .cloned()
    }
}
