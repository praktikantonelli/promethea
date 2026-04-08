use epub::doc::EpubDoc;
use shared_core::ports::filesystem::{FileSystemError, FileSystemPort};
use std::path::Path;

pub struct FileSystem {}

impl FileSystemPort for FileSystem {
    async fn create_file(&self, path: &Path) -> Result<(), FileSystemError> {}

    async fn move_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError> {}

    async fn copy_file(&self, source: &Path, target: &Path) -> Result<(), FileSystemError> {}

    async fn delete_file(&self, path: &Path) -> Result<(), FileSystemError> {}

    fn extract_title_from_epub(&self, path: &Path) -> Result<String, FileSystemError> {
        let doc = EpubDoc::new(path)?;
        doc.get_title()
    }

    fn extract_author_from_epub(&self, path: &Path) -> Result<String, FileSystemError> {
        let doc = EpubDoc::new(path)?;
        let authors = doc
            .metadata
            .iter()
            .filter(|item| item.property == "creator")
            .map(|item| item.value.clone())
            .collect::<Vec<String>>();
        Ok(authors.first()?)
    }
}
