use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct BookRecord {
    book_id: i64,
    title: String,
    sort: String,
    #[sqlx(json)]
    authors: Vec<String>,
    #[sqlx(json)]
    authors_sort: Vec<String>,
    #[sqlx(json)]
    series_and_volume: Vec<SeriesAndVolume>,
    number_of_pages: u32,
    goodreads_id: u64,
    date_added: DateTime<Utc>,
    date_published: DateTime<Utc>,
    date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct SeriesAndVolume {
    series: String,
    sort: String,
    volume: f64,
}
