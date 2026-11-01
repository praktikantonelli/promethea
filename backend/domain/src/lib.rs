//! `domain`

use std::collections::HashMap;
use uuid::Uuid;

/// A representation of a work of literature.
pub struct Book {
    /// A unique identifier for books of the same name
    book_id: BookID,
    /// The book's title
    title: String,
    /// A possible subtitle
    subtitle: Option<String>,
    /// The list of authors
    authors: Vec<Author>,
    /// The number of pages in the book
    number_of_pages: u16,
    /// A collection of all series this book belongs to
    series: HashMap<SeriesID, SeriesVolume>,
    /// All formats in which a work of literature is in the library
    formats: Vec<Format>,
}

/// An individual that has contributed to a book.
pub struct Author {
    /// A unique identifier for authors of the same name
    author_id: AuthorID,
    /// The full name of the author
    name: String,
}

/// A series or universe of multiple books
pub struct Series {
    /// A unique identifier for series of the same name
    series_id: SeriesID,
    /// The title of the series
    title: String,
}

/// A newtype for the book ID
pub struct BookID(Uuid);

/// A newtype for the author ID
pub struct AuthorID(Uuid);

/// A newtype for the series ID
pub struct SeriesID(Uuid);

/// The index a book has in a series, must be non-negative!
pub struct SeriesVolume(f32);

/// Supported formats of a literary work in the library
pub enum Format {
    /// A digital version of a literary work
    EBook(EBook),
    /// An audio version of a literary work
    Audiobook(Audiobook),
    /// A digital representation of a physical book
    Physical(PhysicalBook),
}

/// A digital book
pub struct EBook {}

/// An audiobook
pub struct Audiobook {}

/// A physical book
pub struct PhysicalBook {}
