use crate::ports::{metadata::MetadataProviderPort, repository::BookRepositoryPort};
use std::sync::Arc;

pub struct AddBookUseCase {
    repository: Arc<dyn BookRepositoryPort>,
    metadata: Arc<dyn MetadataProviderPort>,
}

impl AddBookUseCase {
    pub async fn execute(&self) -> Result<(), AddBookError> {}
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum AddBookError {}
