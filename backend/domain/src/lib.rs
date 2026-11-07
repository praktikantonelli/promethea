//! `domain`, the domain model of the project.

/// The domain model concerning authors.
pub mod authors;
pub use authors::Author;

/// The domain model concerning books and literary works.
pub mod books;
pub use books::LiteraryWork;

/// The domain model concerning book series.
pub mod series;
pub use series::Series;
