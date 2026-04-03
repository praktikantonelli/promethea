use crate::domain::{metadata::BookMetadata, records::GoodreadsId};

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
pub trait MetadataProviderPort {
    fn create() -> Self;

    async fn fetch_goodreads_id(
        &self,
        title: String,
        author: String,
    ) -> Result<Option<GoodreadsId>, FetchMetadataError>;

    async fn fetch_metadata(
        &self,
        goodreads_id: GoodreadsId,
    ) -> Result<BookMetadata, FetchMetadataError>;
}

/// Error fetching metadata for book
#[derive(thiserror::Error, Debug)]
enum FetchMetadataError {}
