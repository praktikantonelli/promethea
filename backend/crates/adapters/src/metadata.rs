use reqwest::{ClientBuilder, header};
use scraper::{Html, Selector};
use shared_core::domain::metadata::{BookRecord, GoodreadsId};
use shared_core::ports::metadata::MetadataProviderPort;
use urlencoding::encode;

pub struct MetadataProvider {
    http_client: reqwest::Client,
}

impl MetadataProviderPort for MetadataProvider {
    fn create() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static(
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            ),
        );
        headers.insert(
            header::ACCEPT_LANGUAGE,
            header::HeaderValue::from_static("en-US,en;q=0.9"),
        );
        let client = ClientBuilder::new()
            .user_agent(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
             AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/120.0 Safari/537.36",
            )
            .default_headers(headers)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(25))
            .redirect(Policy::limited(10))
            .pool_max_idle_per_host(1)
            .pool_idle_timeout(Duration::from_secs(30))
            .build();

        client
            .map(|http_client| Self { http_client })
            .map_err(|err| format!("Failed to create HTTP request client for scraping: {err}"))
    }

    async fn fetch_goodreads_id(&self) -> Result<GoodreadsId, FetchMetadataError> {}

    async fn fetch_metadata(
        &self,
        goodreads_id: GoodreadsId,
    ) -> Result<BookRecord, FetchMetadataError> {
    }
}

impl MetadataProvider {
    async fn search(
        &self,
        title: String,
        author: String,
    ) -> Result<Option<GoodreadsId>, SearchError> {
        let query = format!("{} {}", title, author);
        let url = format!("https://www.goodreads.com/search?q={}", encode(&query));

        let document =
            Html::parse_document(&self.http_client.get(&url).send().await?.text().await?);
        let title_selector = Selector::parse(r#"a[class="bookTitle"]"#)?;
        let author_selector = Selector::parse(r#"a[class="authorName"]"#)?;

        for (title, author) in document
            .select(&title_selector)
            .zip(document.select(&author_selector))
        {
            let found_title = title.text().collect::<String>();
            let found_author = author.text().collect::<String>();
            let found_link = title.value().attr("href")?;
            let found_id = extract_goodreads_id_from_link(found_link)?;

            if matches(&found_title, &title) && matches(&found_author, &author) {
                return Ok(Some(found_id));
            }
        }
        Ok(None)
    }
}

fn extract_goodreads_id_from_link(link: &str) -> Result<GoodreadsId, ExtractError> {
    url.splitn(4, '/')
        .nth(3)
        .unwrap_or("")
        .split('?')
        .next()?
        .chars()
        .take_while(|character| character.is_numeric())
        .collect::<String>()
}

pub fn matches(str1: &str, str2: &str) -> bool {
    let str1 = str1
        .chars()
        .filter(|character| character.is_alphanumeric())
        .collect::<String>();
    let str2 = str2
        .chars()
        .filter(|character| character.is_alphanumeric())
        .collect::<String>();

    str1.to_lowercase().contains(&str2.to_lowercase())
}

#[derive(thiserror::Error, Debug)]
enum SearchError {}

#[derive(thiserror::Error, Debug)]
enum ExtractError {}
