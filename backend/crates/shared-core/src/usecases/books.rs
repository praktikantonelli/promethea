use crate::ports::repository::BookRepositoryPort;
use std::sync::Arc;

pub struct AddBookUseCase {
    repository: Arc<dyn BookRepositoryPort>,
}
