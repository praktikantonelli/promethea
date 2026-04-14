use async_trait::async_trait;
use std::path::PathBuf;

use crate::domain::{
    metadata::{BookMetadata, GoodreadsId},
    repository::{AuthorItem, BookItem, SeriesAndVolumeItem},
};

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
#[async_trait]
pub trait BookRepositoryPort {
    async fn close(&self);

    async fn fetch_all_books(&self) -> Result<Vec<BookItem>, FetchError>;

    async fn try_fetch_author_sort(&self, author_name: &str) -> Result<Option<String>, FetchError>;

    async fn try_fetch_series_sort(&self, series_title: &str)
    -> Result<Option<String>, FetchError>;

    async fn insert_book(&self, book: BookMetadata) -> Result<(), InsertBookError>;

    async fn update_book(&self, book: BookItem) -> Result<(), UpdateError>;

    async fn update_series(&self, series: SeriesAndVolumeItem) -> Result<(), UpdateError>;

    async fn update_author(&self, author: AuthorItem) -> Result<(), UpdateError>;
}

/// Error when trying to add a new book to the repository
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum InsertBookError {
    /// The book already exists in the repository
    #[error("book with Goodreads ID `{goodreads_id}` already exists")]
    Conflict {
        /// Goodreads ID of the duplicated book
        goodreads_id: GoodreadsId,
    },

    /// The repository is unavailable
    #[error("storage unavailable")]
    Unavailable,
}

/// Error when trying to open an existing repository
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum OpenRepositoryError {
    /// Path not available (e.g., file doesn't exist)
    #[error("repository at `{path}` not found")]
    Path { path: PathBuf },

    #[error("failed to run initialize repository")]
    Initialization,
}

/// Error fetching data from the repository
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum FetchError {
    #[error("failed to run fetch query: `{message}`")]
    Generic { message: String },
}

/// Error updating data in the repository
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum UpdateError {
    #[error("failed to run update query: `{message}`")]
    Generic { message: String },
}
