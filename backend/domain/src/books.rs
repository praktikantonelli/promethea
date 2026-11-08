use super::authors::Author;
use super::series::{SeriesID, Volume};
use std::path::PathBuf;
use uuid::Uuid;
/// A representation of a work of literature.
///
/// A literary work does not take editions, revisions and publishers into account.
#[allow(dead_code, reason = "unused only for now")]
#[derive(Debug)]
pub struct LiteraryWork {
    /// A unique identifier for books of the same name.
    book_id: BookID,
    /// The literary work's title.
    title: String,
    /// A possible subtitle.
    subtitle: Option<String>,
    /// The list of authors.
    authors: Vec<Author>,
    /// A collection of all series this literary work belongs to.
    series: Vec<(SeriesID, Volume)>,
    /// All formats in which a work of literature is in the library.
    formats: Vec<Format>,
}

/// A newtype for the book ID.
#[allow(dead_code, reason = "unused only for now")]
#[derive(Debug, Clone, Copy)]
pub struct BookID(Uuid);

/// Supported formats of a literary work in the library.
#[allow(dead_code, reason = "unused only for now")]
#[derive(Debug)]
pub enum Format {
    /// A digital version of a literary work.
    EBook(EBook),
    /// An audio version of a literary work.
    Audiobook(Audiobook),
    /// A digital representation of a physical book.
    Physical(PhysicalBook),
}

/// A digital book.
#[allow(dead_code, reason = "unused only for now")]
#[derive(Debug)]
pub struct EBook {
    /// the path at which the ebook is stored.
    path: PathBuf,
}

/// An audiobook.
#[allow(dead_code, reason = "unused only for now")]
#[derive(Debug)]
pub struct Audiobook {
    /// the path at which the audiobook is stored.
    path: PathBuf,
}

/// A physical book.
#[allow(dead_code, reason = "unused only for now")]
#[allow(missing_copy_implementations, reason = "only empty for now")]
#[derive(Debug)]
pub struct PhysicalBook;
