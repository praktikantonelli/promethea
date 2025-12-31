use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct BookRecord {
    book_id: i64,
    title: String,
    sort: String,
    #[sqlx(json)]
    authors: Vec<AuthorRecord>,
    #[sqlx(json)]
    series_and_volume: Vec<SeriesAndVolumeRecord>,
    number_of_pages: u32,
    goodreads_id: u64,
    date_added: DateTime<Utc>,
    date_published: DateTime<Utc>,
    date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct AuthorRecord {
    name: String,
    sort: String,
    goodreads_id: u64,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct SeriesAndVolumeRecord {
    series: String,
    sort: String,
    volume: f64,
}

#[derive(Debug, thiserror::Error)]
pub enum InsertBookError {
    #[error("book already exists (goodreads_id={0})")]
    BookAlreadyExists(u64),

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}
