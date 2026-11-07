use uuid::Uuid;

/// An individual that has contributed to a book.
pub struct Author {
    /// A unique identifier for authors of the same name.
    author_id: AuthorID,
    /// The full name of the author.
    name: String,
}

/// A newtype for the author ID.
pub struct AuthorID(Uuid);
