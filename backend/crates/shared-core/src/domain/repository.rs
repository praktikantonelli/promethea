use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookItem {
    pub book_id: i64,
    pub title: String,
    pub sort: String,
    pub authors: Vec<AuthorItem>,
    pub series_and_volume: Vec<SeriesAndVolumeItem>,
    pub number_of_pages: i64,
    pub goodreads_id: i64,
    pub date_added: NaiveDateTime,
    pub date_published: Option<NaiveDateTime>,
    pub date_modified: NaiveDateTime,
}

impl BookItem {
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
        authors: Vec<AuthorItem>,
        series_and_volume: Vec<SeriesAndVolumeItem>,
        number_of_pages: i64,
        goodreads_id: i64,
        date_added: NaiveDateTime,
        date_published: Option<NaiveDateTime>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorItem {
    pub name: String,
    pub sort: String,
    pub goodreads_id: i64,
}

impl AuthorItem {
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesAndVolumeItem {
    pub series: String,
    pub sort: String,
    pub volume: f64,
    pub goodreads_id: i64,
}

impl SeriesAndVolumeItem {
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

#[derive(Debug, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub struct GoodreadsId(pub i64);

impl Display for GoodreadsId {
    #[inline]
    #[allow(
        clippy::min_ident_chars,
        reason = "signature is defined by Display trait"
    )]
    #[allow(
        clippy::absolute_paths,
        reason = "importing would override default Result"
    )]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GoodreadsId {
    #[must_use]
    #[inline]
    pub const fn new(id: i64) -> Self {
        Self(id)
    }
}
