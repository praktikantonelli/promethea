#[non_exhaustive]
#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct BookRecord {
    pub book_id: i64,
    pub title: String,
    pub sort: String,
    #[sqlx(json)]
    pub authors: Vec<AuthorRecord>,
    #[sqlx(json)]
    pub series_and_volume: Vec<SeriesAndVolumeRecord>,
    pub number_of_pages: i64,
    pub goodreads_id: i64,
    pub date_added: NaiveDateTime,
    pub date_published: NaiveDateTime,
    pub date_modified: NaiveDateTime,
}

#[non_exhaustive]
#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct AuthorRecord {
    pub name: String,
    pub sort: String,
    pub goodreads_id: i64,
}

#[non_exhaustive]
#[derive(Serialize, Debug, Deserialize, Clone, sqlx::FromRow)]
pub struct SeriesAndVolumeRecord {
    pub series: String,
    pub sort: String,
    pub volume: f64,
    pub goodreads_id: i64,
}
