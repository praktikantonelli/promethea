use async_trait::async_trait;

use crate::domain::metadata::BookMetadata;
use crate::domain::repository::GoodreadsId;

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
#[async_trait]
pub trait MetadataProviderPort {
    async fn fetch_goodreads_id(
        &self,
        title: &str,
        author: &str,
    ) -> Result<Option<GoodreadsId>, FetchMetadataError>;

    async fn fetch_metadata(
        &self,
        goodreads_id: GoodreadsId,
    ) -> Result<BookMetadata, FetchMetadataError>;
}

/// Error fetching metadata for book
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum FetchMetadataError {
    #[error("failed to fetch Goodreads ID for title `{title}` and author `{author}`")]
    GoodreadsId { title: String, author: String },
    #[error("failed to fetch metadata for Goodreads ID `{goodreads_id}`")]
    Metadata { goodreads_id: GoodreadsId },
    #[error("failed to extract value for key `{key}`: `{message}`")]
    Extraction { key: String, message: String },
    #[error("failed setup in stage `{stage}`: `{message}`")]
    Setup { stage: String, message: String },
    #[error("failed request `{request_type} `{url}`: `{message}`")]
    Request {
        request_type: String,
        url: String,
        message: String,
    },
}
