use uuid::Uuid;

/// An individual that has contributed to a book.
#[allow(dead_code, reason = "unused only for now")]
#[derive(Debug)]
pub struct Author {
    /// A unique identifier for authors of the same name.
    author_id: AuthorID,
    /// The full name of the author.
    name: String,
}

/// A newtype for the author ID.
#[allow(dead_code, reason = "unused only for now")]
#[derive(Clone, Copy, Debug)]
pub struct AuthorID(Uuid);
