use crate::domain::repository::GoodreadsId;
use chrono::{DateTime, Utc};

/// The primary data structure containing the metadata of a book.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct BookMetadata {
    /// The main title of the book.
    pub title: String,
    /// The publication date of the book, represented as a UTC datetime.
    pub publication_date: Option<DateTime<Utc>>,
    /// A list of contributors to the book, each represented as a `BookContributor`.
    pub contributors: Vec<BookContributor>,
    /// A list of series information, if the book is part of a series, represented as a `BookSeries`.
    pub series: Vec<BookSeries>,
    /// The number of pages in the book, if available.
    pub number_of_pages: Option<i64>,
    /// A URL to an image of the book's cover, if available.
    pub image_url: Option<String>,
    /// The ID with which the book's metadata has been fetched
    pub goodreads_id: GoodreadsId,
}

/// Represents an individual who contributed to the book, such as an author or editor.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct BookContributor {
    /// The name of the contributor.
    pub name: String,
    /// The role of the contributor, such as "Author" or "Illustrator".
    pub role: String,
    /// The Goodreads ID of the contributor
    pub goodreads_id: GoodreadsId,
}

/// Represents series information for a book, including the series title and book's position within the series.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct BookSeries {
    /// The title of the series.
    pub title: String,
    /// The position of the book within the series, represented as a float to accommodate cases like "1.5".
    pub number: f32,
    /// The Goodreads ID of the series
    pub goodreads_id: GoodreadsId,
}
