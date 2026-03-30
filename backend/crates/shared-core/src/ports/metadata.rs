use crate::domain::metadata::{BookRecord, GoodreadsId};

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
pub trait MetadataProviderPort {
    fn create() -> Self;

    async fn fetch_goodreads_id(&self) -> Result<GoodreadsId, FetchMetadataError>;

    async fn fetch_metadata(
        &self,
        goodreads_id: GoodreadsId,
    ) -> Result<BookRecord, FetchMetadataError>;
}

/// Error fetching metadata for book
#[derive(thiserror::Error, Debug)]
enum FetchMetadataError {}
