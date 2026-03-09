#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum InsertBookError {
    #[error("book already exists (goodreads_id={0})")]
    BookAlreadyExists(i64),

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}
