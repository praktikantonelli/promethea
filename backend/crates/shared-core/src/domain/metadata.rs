use crate::domain::repository::GoodreadsId;
use chrono::NaiveDateTime;

/// The primary data structure containing the metadata of a book.
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub struct BookMetadata {
    /// The main title of the book.
    pub title: String,
    /// The publication date of the book, represented as a UTC datetime.
    pub publication_date: Option<NaiveDateTime>,
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

impl BookMetadata {
    #[must_use]
    #[inline]
    pub fn new(
        title: &str,
        publication_date: Option<NaiveDateTime>,
        contributors: Vec<BookContributor>,
        series: Vec<BookSeries>,
        number_of_pages: Option<i64>,
        image_url: Option<String>,
        goodreads_id: GoodreadsId,
    ) -> Self {
        Self {
            title: title.to_owned(),
            publication_date,
            contributors,
            series,
            number_of_pages,
            image_url,
            goodreads_id,
        }
    }
}

/// Represents an individual who contributed to the book, such as an author or editor.
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub struct BookContributor {
    /// The name of the contributor.
    pub name: String,
    /// The role of the contributor, such as "Author" or "Illustrator".
    pub role: String,
    /// The Goodreads ID of the contributor
    pub goodreads_id: GoodreadsId,
}

impl BookContributor {
    #[must_use]
    #[inline]
    pub fn new(name: &str, role: &str, goodreads_id: GoodreadsId) -> Self {
        Self {
            name: name.to_owned(),
            role: role.to_owned(),
            goodreads_id,
        }
    }
}

/// Represents series information for a book, including the series title and book's position within the series.
#[non_exhaustive]
#[derive(Debug, PartialEq, Clone)]
pub struct BookSeries {
    /// The title of the series.
    pub title: String,
    /// The position of the book within the series, represented as a float to accommodate cases like "1.5".
    pub number: f32,
    /// The Goodreads ID of the series
    pub goodreads_id: GoodreadsId,
}

impl BookSeries {
    #[must_use]
    #[inline]
    pub fn new(title: &str, number: f32, goodreads_id: GoodreadsId) -> Self {
        Self {
            title: title.to_owned(),
            number,
            goodreads_id,
        }
    }
}
