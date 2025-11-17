pub trait RequestState {}
pub struct EmptyState;
pub struct IdState(String);
pub struct TitleState(String);
pub struct TitleAndAuthorState(String, String);

impl RequestState for EmptyState {}
impl RequestState for IdState {}
impl RequestState for TitleState {}
impl RequestState for TitleAndAuthorState {}

pub struct MetadataRequestBuilder<T: RequestState> {
    state: T,
}

impl Default for MetadataRequestBuilder<EmptyState> {
    fn default() -> Self {
        MetadataRequestBuilder::new()
    }
}

impl MetadataRequestBuilder<EmptyState> {
    pub fn new() -> Self {
        MetadataRequestBuilder { state: EmptyState }
    }

    pub fn with_id(self, id: &str) -> MetadataRequestBuilder<IdState> {
        MetadataRequestBuilder {
            state: IdState(id.to_string()),
        }
    }

    pub fn with_title(self, title: &str) -> MetadataRequestBuilder<TitleState> {
        MetadataRequestBuilder {
            state: TitleState(title.to_string()),
        }
    }
}

impl MetadataRequestBuilder<TitleState> {
    pub fn with_author(self, author: &str) -> MetadataRequestBuilder<TitleAndAuthorState> {
        MetadataRequestBuilder {
            state: TitleAndAuthorState(self.state.0, author.to_string()),
        }
    }
}
