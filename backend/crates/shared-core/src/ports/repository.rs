use std::path::Path;

use crate::domain::metadata::{AuthorRecord, BookRecord, SeriesAndVolumeRecord};

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
pub trait BookRepositoryPort: Sized {
    async fn open(path: &Path) -> Result<Self, OpenRepositoryError>;

    async fn close(&self);

    async fn fetch_all_books(&self) -> Result<Vec<BookRecord>, FetchError>;

    async fn try_fetch_author_sort(&self, author_name: &str) -> Result<Option<String>, FetchError>;

    async fn try_fetch_series_sort(&self, series_title: &str)
    -> Result<Option<String>, FetchError>;

    async fn insert_book(&self, book: BookRecord) -> Result<(), InsertBookError>;

    async fn update_book(&self, book: BookRecord) -> Result<(), UpdateError>;

    async fn update_series(&self, series: SeriesAndVolumeRecord) -> Result<(), UpdateError>;

    async fn update_author(&self, author: AuthorRecord) -> Result<(), UpdateError>;
}

/// Error when trying to add a new book to the repository
#[derive(thiserror::Error, Debug)]
enum InsertBookError {
    /// The book already exists in the repository
    #[error("book with title `{title}` already exists")]
    Conflict {
        /// Title of the duplicated book
        title: String,
    },

    /// The repository is unavailable
    #[error("storage unavailable")]
    Unavailable,
}

/// Error when trying to open an existing repository
#[derive(thiserror::Error, Debug)]
enum OpenRepositoryError {}

/// Error fetching data from the repository
#[derive(thiserror::Error, Debug)]
enum FetchError {}

/// Error updating data in the repository
#[derive(thiserror::Error, Debug)]
enum UpdateError {}
