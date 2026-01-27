use crate::scraping::errors::ScraperError;
use crate::scraping::metadata_fetcher::BookMetadata;
use crate::scraping::metadata_fetcher::{
    extract_amazon_id, extract_book_metadata, extract_contributors, extract_image_url,
    extract_page_count, extract_publication_date, extract_series, extract_title_and_subtitle,
};
use core::time::Duration;
use reqwest::redirect::Policy;

use reqwest::{ClientBuilder, header};
use scraper::{Html, Selector};
use urlencoding::encode;

pub struct MetadataRequestClient {
    /// A HTTP client used to execute all GET requests for querying Goodreads
    http_client: reqwest::Client,
}

impl MetadataRequestClient {
    /// Create a new HTTP request client, to be used for all subsequent metadata scraping requests
    /// # Errors
    /// Fails in case any of the reqwest `ClientBuilder` methods fail
    #[allow(
        clippy::missing_inline_in_public_items,
        reason = "Called once per program run"
    )]
    pub fn new() -> Result<Self, String> {
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

    /// Given title and author, fetches Goodreads ID. First attempts to only use title to keep the
    /// query as concise as possible, only uses author as a fallback
    /// # Errors
    /// The function fails if the search for the book fails.
    #[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
    async fn fetch_goodreads_id(
        &self,
        title: &str,
        author: &str,
    ) -> Result<Option<String>, ScraperError> {
        let results_with_title = self.search_books(title).await?;

        for (found_title, found_author, found_id) in results_with_title {
            if matches(&found_title, title) && matches(&found_author, author) {
                return Ok(Some(found_id));
            }
        }

        let results_with_title_author = self.search_books(&format!("{title} {author}")).await?;

        for (found_title, found_author, found_id) in results_with_title_author {
            if matches(&found_title, title) && matches(&found_author, author) {
                return Ok(Some(found_id));
            }
        }

        Ok(None)
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

    /// Fetches metadata for a given title and author of a book
    /// # Errors
    /// Returns an error if the request fails
    #[inline]
    pub async fn fetch_metadata(
        &self,
        title: &str,
        author: &str,
    ) -> Result<Option<BookMetadata>, ScraperError> {
        let goodreads_id = self.fetch_goodreads_id(title, author).await?;
        match goodreads_id {
            Some(id) => Ok(Some(self.get_metadata(&id).await?)),
            None => Ok(None),
        }
    }

    /// Handles the HTTP requests and subsequent extraction of data from the response
    /// # Errors
    /// Returns an error if any of the individual methods returns an error
    async fn get_metadata(&self, goodreads_id: &str) -> Result<BookMetadata, ScraperError> {
        let metadata = extract_book_metadata(goodreads_id).await?;
        let amazon_id = extract_amazon_id(&metadata, goodreads_id)?;

        let (title, _subtitle) = extract_title_and_subtitle(&metadata, &amazon_id)?;
        let image_url = extract_image_url(&metadata, &amazon_id);
        let contributors = extract_contributors(&metadata, &amazon_id);
        let publication_date = extract_publication_date(&metadata, &amazon_id);
        let page_count = extract_page_count(&metadata, &amazon_id);
        let series = extract_series(&metadata, &amazon_id);
        let goodreads_id = Some(goodreads_id.to_owned());

        Ok(BookMetadata {
            title,
            publication_date,
            contributors,
            series,
            page_count,
            image_url,
            goodreads_id,
        })
    }
}

/// Helper function to determine if two strings match, ignoring upper and lower case as well as
/// interpunctuations in initials.
#[inline]
#[must_use]
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

/// Tries and extracts the Goodreads ID out of a Goodreads URL
#[inline]
#[must_use]
pub fn extract_goodreads_id(url: &str) -> String {
    url.splitn(4, '/')
        .nth(3)
        .unwrap_or("")
        .split('?')
        .next()
        .unwrap_or("")
        .chars()
        .take_while(|character| character.is_numeric())
        .collect::<String>()
}
