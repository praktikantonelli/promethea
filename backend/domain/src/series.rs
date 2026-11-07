use uuid::Uuid;

/// A series or universe of multiple books.
pub struct Series {
    /// A unique identifier for series of the same name.
    series_id: SeriesID,
    /// The title of the series.
    title: String,
}

/// A newtype for the series ID.
pub struct SeriesID(Uuid);

/// The index a book has in a series, must be non-negative!
#[allow(dead_code, reason = "unused only for now")]
#[derive(Debug, Clone, Copy)]
pub struct Volume(f32);
