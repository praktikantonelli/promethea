use std::path::Path;

pub trait DatabasePort: Sized {
    #[allow(async_fn_in_trait, reason = "Only used in my own code")]
    async fn init(path: &Path) -> Result<Self, sqlx::Error>; // TODO: replace sqlx:Error with own type
}
