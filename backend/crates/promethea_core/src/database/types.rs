use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct BookRecord {
    pub book_id: i64,
    pub title: String,
    pub sort: String,
    #[sqlx(json)]
    pub authors: Vec<AuthorRecord>,
    #[sqlx(json)]
    pub series_and_volume: Vec<SeriesAndVolumeRecord>,
    pub number_of_pages: u32,
    pub goodreads_id: u64,
    pub date_added: DateTime<Utc>,
    pub date_published: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
}

#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct AuthorRecord {
    pub name: String,
    pub sort: String,
    pub goodreads_id: u64,
}

#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct SeriesAndVolumeRecord {
    pub series: String,
    pub sort: String,
    pub volume: f64,
    pub goodreads_id: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum InsertBookError {
    #[error("book already exists (goodreads_id={0})")]
    BookAlreadyExists(u64),

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}
