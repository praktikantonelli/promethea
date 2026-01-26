use crate::scraper::errors::ScraperError;
use crate::scraper::goodreads_id_fetcher::extract_goodreads_id;
use core::time::Duration;
use reqwest::redirect::Policy;

use reqwest::{ClientBuilder, header};
use scraper::{Html, Selector};
use urlencoding::encode;

pub struct MetadataRequestClient {
    http_client: reqwest::Client,
}

impl MetadataRequestClient {
    /// Create a new HTTP request client, to be used for all subsequent metadata scraping requests
    fn new() -> Result<Self, String> {
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

    /// Use title and author(s) to fetch a book's Goodreads ID
    async fn fetch_goodreads_id(
        &self,
        title: String,
        authors: Vec<String>,
    ) -> Result<Option<i32>, String> {
        let query = format!("{title} {}", authors.join(" "));
        let result = &self
            .http_client
            .get(format!("https://goodreads.com/search?q={query}"))
            .send()
            .await?;
    }

    /// Tries to query Goodreads with a given query (e.g., title and author as one string).
    /// # Errors
    /// Returns an error if fetching or parsing the website fails, or if no link to the specified query
    /// can be extracted
    async fn search_books(
        &self,
        query: &str,
    ) -> Result<Vec<(String, String, String)>, ScraperError> {
        let url = format!("https://www.goodreads.com/search?q={}", encode(query));

        //let document = Html::parse_document(&get(&url).await?.text().await?);
        let document =
            Html::parse_document(&self.http_client.get(&url).send().await?.text().await?);
        let title_selector = Selector::parse(r#"a[class="bookTitle"]"#)?;
        let author_selector = Selector::parse(r#"a[class="authorName"]"#)?;

        let mut results = Vec::new();

        for (title, author) in document
            .select(&title_selector)
            .zip(document.select(&author_selector))
        {
            let found_title = title.text().collect::<String>();
            let found_author = author.text().collect::<String>();
            let found_link = title.value().attr("href").ok_or(ScraperError::ParseError(
                "Failed to extract link from search result".to_owned(),
            ))?;
            let found_id = extract_goodreads_id(found_link);

            results.push((found_title, found_author, found_id));
        }
        Ok(results)
    }
}
