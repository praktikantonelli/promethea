use std::path::Path;

use crate::domain::metadata::BookRecord;

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
pub trait DatabasePort: Sized {
    async fn init(path: &Path) -> Result<Self, sqlx::Error>; // TODO: replace sqlx:Error with own type

    async fn close(&self);

    async fn fetch_all_books(&self) -> Result<Vec<BookRecord>, sqlx::Error>;

    async fn try_fetch_author_sort(&self, author_name: &str)
    -> Result<Option<String>, sqlx::Error>;

    async fn try_fetch_series_sort(
        &self,
        series_title: &str,
    ) -> Result<Option<String>, sqlx::Error>;

    async fn insert_book(&self, book: BookRecord) -> Result<(), sqlx::Error>;
}
