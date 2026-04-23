use crate::{
    domain::repository::BookItem,
    ports::repository::{BookRepositoryPort, FetchError},
};
use std::sync::Arc;

pub struct FetchBooksUseCase {
    /// book repository adapter
    repository: Arc<dyn BookRepositoryPort + Send + Sync>,
}

impl FetchBooksUseCase {
    #[inline]
    pub fn new(repository: Arc<dyn BookRepositoryPort + Send + Sync>) -> Self {
        Self { repository }
    }

    /// Tries to fetch all books from the repository
    ///
    /// # Errors
    /// Passes on any error occurring in the repository port
    #[inline]
    pub async fn execute(&self) -> Result<FetchBooksOutput, FetchBooksError> {
        let books = self.repository.fetch_all_books().await?;
        Ok(FetchBooksOutput(books))
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum FetchBooksError {
    #[error("failed to fetch books from repository")]
    RepositoryError(#[from] FetchError),
}

#[non_exhaustive]
pub struct FetchBooksOutput(pub Vec<BookItem>);
