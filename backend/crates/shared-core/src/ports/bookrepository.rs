use std::path::Path;

use crate::domain::metadata::BookRecord;

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
pub trait BookRepositoryPort: Sized {
    async fn create_new(path: &Path) -> Result<Self, CreateNewError>;

    async fn open_existing(path: &Path) -> Result<Self, OpenExistingError>;

    async fn close(&self) -> Result<(), CloseError>;

    async fn fetch_all_books(&self) -> Result<Vec<BookRecord>, sqlx::Error>;

    async fn try_fetch_author_sort(&self, author_name: &str)
    -> Result<Option<String>, sqlx::Error>;

    async fn try_fetch_series_sort(
        &self,
        series_title: &str,
    ) -> Result<Option<String>, sqlx::Error>;

    async fn insert_book(&self, book: BookRecord) -> Result<(), InsertBookError>;
}

#[derive(thiserror::Error, Debug)]
enum InsertBookError {
    #[error("book with title `{title}` already exists")]
    Conflict { title: String },

    #[error("storage unavailable")]
    Unavailable,
}

#[derive(thiserror::Error, Debug)]
enum CreateNewError {}

#[derive(thiserror::Error, Debug)]
enum OpenExistingError {}

#[derive(thiserror::Error, Debug)]
enum CloseError {}
