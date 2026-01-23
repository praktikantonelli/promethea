use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct BookRecord {
    pub book_id: i64,
    pub title: String,
    pub sort: String,
    #[sqlx(json)]
    pub authors: Vec<AuthorRecord>,
    #[sqlx(json)]
    pub series_and_volume: Vec<SeriesAndVolumeRecord>,
    pub number_of_pages: i64,
    pub goodreads_id: i64,
    pub date_added: NaiveDateTime,
    pub date_published: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

impl BookRecord {
    #[allow(
        clippy::too_many_arguments,
        reason = "Constructor, cannot have fewer arguments"
    )]
    #[must_use]
    #[inline]
    pub const fn new(
        book_id: i64,
        title: String,
        sort: String,
        authors: Vec<AuthorRecord>,
        series_and_volume: Vec<SeriesAndVolumeRecord>,
        number_of_pages: i64,
        goodreads_id: i64,
        date_added: NaiveDateTime,
        date_published: NaiveDateTime,
        date_modified: NaiveDateTime,
    ) -> Self {
        Self {
            book_id,
            title,
            sort,
            authors,
            series_and_volume,
            number_of_pages,
            goodreads_id,
            date_added,
            date_published,
            date_modified,
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct AuthorRecord {
    pub name: String,
    pub sort: String,
    pub goodreads_id: i64,
}

impl AuthorRecord {
    #[must_use]
    #[inline]
    pub const fn new(name: String, sort: String, goodreads_id: i64) -> Self {
        Self {
            name,
            sort,
            goodreads_id,
        }
    }
}

#[non_exhaustive]
#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct SeriesAndVolumeRecord {
    pub series: String,
    pub sort: String,
    pub volume: f64,
    pub goodreads_id: i64,
}

impl SeriesAndVolumeRecord {
    #[inline]
    #[must_use]
    pub const fn new(series: String, sort: String, volume: f64, goodreads_id: i64) -> Self {
        Self {
            series,
            sort,
            volume,
            goodreads_id,
        }
    }
}

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum InsertBookError {
    #[error("book already exists (goodreads_id={0})")]
    BookAlreadyExists(i64),

    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),
}
