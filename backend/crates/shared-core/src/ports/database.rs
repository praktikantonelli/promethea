use std::path::Path;

#[allow(async_fn_in_trait, reason = "Only used in my own code")]
pub trait DatabasePort: Sized {
    async fn init(path: &Path) -> Result<Self, sqlx::Error>; // TODO: replace sqlx:Error with own type
}
