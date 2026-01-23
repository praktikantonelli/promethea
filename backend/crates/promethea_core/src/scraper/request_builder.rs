use crate::scraper::{
    errors::ScraperError,
    goodreads_id_fetcher::{
        fetch_id_from_isbn, fetch_id_from_title, fetch_id_from_title_and_author, verify_id_exists,
    },
    metadata_fetcher::{BookMetadata, fetch_metadata},
};

pub trait RequestState {}
#[allow(clippy::exhaustive_structs, reason = "Empty state will remain empty")]
pub struct EmptyState;
pub struct IdState(String);
pub struct IsbnState(String);
pub struct TitleState(String);
pub struct TitleWithAuthorState(String, String);

impl RequestState for EmptyState {}
impl RequestState for IdState {}
impl RequestState for IsbnState {}
impl RequestState for TitleState {}
impl RequestState for TitleWithAuthorState {}

/// Builder for constructing a metadata request.
pub struct MetadataRequestBuilder<T: RequestState> {
    /// Represents the type of data used to fetch the metadata: no data, Goodreads ID, ISBN, title
    /// or title and author
    state: T,
}

impl Default for MetadataRequestBuilder<EmptyState> {
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    fn default() -> Self {
        Self::new()
    }
}

impl MetadataRequestBuilder<EmptyState> {
    /// Constructor function for default empty state. Use this in combination with any of the other
    /// generic implementations to construct a new `MetadataRequestBuilder` and modify its state.
    const fn new() -> Self {
        Self { state: EmptyState }
    }

    #[must_use]
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub fn with_id(self, id: &str) -> MetadataRequestBuilder<IdState> {
        MetadataRequestBuilder {
            state: IdState(id.to_owned()),
        }
    }

    #[must_use]
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub fn with_isbn(self, isbn: &str) -> MetadataRequestBuilder<IsbnState> {
        MetadataRequestBuilder {
            state: IsbnState(isbn.to_owned()),
        }
    }

    #[must_use]
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub fn with_title(self, title: &str) -> MetadataRequestBuilder<TitleState> {
        MetadataRequestBuilder {
            state: TitleState(title.to_owned()),
        }
    }
}

impl MetadataRequestBuilder<TitleState> {
    #[must_use]
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub fn with_author(self, author: &str) -> MetadataRequestBuilder<TitleWithAuthorState> {
        MetadataRequestBuilder {
            state: TitleWithAuthorState(self.state.0, author.to_owned()),
        }
    }

    /// Execute the HTTP request using the book's title as the input parameter
    /// # Errors
    /// Returns an error if the HTTP request fails or if no Goodreads ID can be extracted from the
    /// response, or if the metadata cannot be fetched  with this Goodreads ID
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub async fn execute(&self) -> Result<Option<BookMetadata>, ScraperError> {
        let title = &self.state.0;
        let goodreads_id = fetch_id_from_title(title).await?;
        match goodreads_id {
            Some(id) => Ok(Some(fetch_metadata(&id).await?)),
            None => Ok(None),
        }
    }
}

impl MetadataRequestBuilder<IdState> {
    /// Execute the HTTP request using the book's Goodreads ID as the input parameter
    /// # Errors
    /// Returns an error if the HTTP request fails
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub async fn execute(&self) -> Result<Option<BookMetadata>, ScraperError> {
        let id = &self.state.0;
        if !verify_id_exists(id).await {
            return Ok(None);
        }
        Ok(Some(fetch_metadata(id).await?))
    }
}

impl MetadataRequestBuilder<IsbnState> {
    /// Execute the HTTP request using the book's ISBN to fetch its Goodreads ID, then use that to
    /// fetch the metadata.
    /// # Errors
    /// Fails if the Goodreads ID cannot be fetched and if the metadata cannot be fetched with the
    /// Goodreads ID
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub async fn execute(&self) -> Result<Option<BookMetadata>, ScraperError> {
        let isbn = &self.state.0;
        let goodreads_id = fetch_id_from_isbn(isbn).await?;
        match goodreads_id {
            Some(id) => Ok(Some(fetch_metadata(&id).await?)),
            None => Ok(None),
        }
    }
}

impl MetadataRequestBuilder<TitleWithAuthorState> {
    /// Execute the HTTP request using the book's title and author to fetch its Goodreads ID, then use that to
    /// fetch the metadata.
    /// # Errors
    /// Fails if the Goodreads ID cannot be fetched and if the metadata cannot be fetched with the
    /// Goodreads ID
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    pub async fn execute(&self) -> Result<Option<BookMetadata>, ScraperError> {
        let title = &self.state.0;
        let author = &self.state.1;
        let goodreads_id = fetch_id_from_title_and_author(title, author).await?;
        match goodreads_id {
            Some(id) => Ok(Some(fetch_metadata(&id).await?)),
            None => Ok(None),
        }
    }
}
