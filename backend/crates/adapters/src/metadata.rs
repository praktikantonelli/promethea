use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime};
use core::time::Duration;
use log::{error, info, warn};
use regex::Regex;
use reqwest::{ClientBuilder, header, redirect::Policy};
use scraper::{Html, Selector};
use serde_json::Value;
use shared_core::domain::metadata::BookMetadata;
use shared_core::domain::metadata::{BookContributor, BookSeries};
use shared_core::domain::repository::GoodreadsId;
use shared_core::ports::metadata::{FetchMetadataError, MetadataProviderPort};
use urlencoding::encode;

pub struct MetadataProvider {
    /// persistent HTTP client to avoid overhead of creating one for every request
    http_client: reqwest::Client,
    /// CSS selector for book titles
    title_selector: Selector,
    /// CSS selector for author names
    author_selector: Selector,
}

#[async_trait]
impl MetadataProviderPort for MetadataProvider {
    #[inline]
    async fn fetch_id_with_title(
        &self,
        title: &str,
    ) -> Result<Option<GoodreadsId>, FetchMetadataError> {
        let document = self.fetch_id(title).await?;

        for title_element in document.select(&self.title_selector) {
            let found_title = title_element.text().collect::<String>().trim().to_owned();
            let found_link = title_element.value().attr("href").ok_or_else(|| {
                FetchMetadataError::Extraction {
                    key: "link".to_owned(),
                    message: "no key named `href`".to_owned(),
                }
            })?;
            let found_id = extract_goodreads_id_from_link(found_link)?;
            if matches(&found_title, title) {
                return Ok(Some(found_id));
            }
        }
        Ok(None)
    }
    #[inline]
    async fn fetch_id_with_title_and_author(
        &self,
        title: &str,
        author: &str,
    ) -> Result<Option<GoodreadsId>, FetchMetadataError> {
        let query = format!("{title} {author}");
        let document = self.fetch_id(&query).await?;

        for (title_element, author_element) in document
            .select(&self.title_selector)
            .zip(document.select(&self.author_selector))
        {
            let found_title = title_element.text().collect::<String>().trim().to_owned();
            let found_author = author_element.text().collect::<String>();
            let found_link = title_element.value().attr("href").ok_or_else(|| {
                FetchMetadataError::Extraction {
                    key: "link".to_owned(),
                    message: "no key named `href`".to_owned(),
                }
            })?;
            let found_id = extract_goodreads_id_from_link(found_link)?;

            if matches(&found_title, title) && matches(&found_author, author) {
                return Ok(Some(found_id));
            }
        }
        Ok(None)
    }

    #[inline]
    async fn fetch_metadata(
        &self,
        goodreads_id: GoodreadsId,
    ) -> Result<BookMetadata, FetchMetadataError> {
        let url = format!("https://www.goodreads.com/book/show/{goodreads_id}");
        let document = Html::parse_document(
            &self
                .http_client
                .get(&url)
                .send()
                .await
                .map_err(|error| FetchMetadataError::Request {
                    request_type: "GET".to_owned(),
                    url,
                    message: error.to_string(),
                })?
                .text()
                .await
                .map_err(|error| FetchMetadataError::Extraction {
                    key: "HTTP response".to_owned(),
                    message: error.to_string(),
                })?,
        );
        let json_selector = Selector::parse(r#"script[id="__NEXT_DATA__"]"#).map_err(|error| {
            FetchMetadataError::Setup {
                stage: "JSON CSS Selector".to_owned(),
                message: error.to_string(),
            }
        })?;
        let json = &document.select(&json_selector).next();

        let json =
            match *json {
                None => {
                    return Err(FetchMetadataError::Extraction {
                        key: "JSON".into(),
                        message: "no metadata JSON in Goodreads page".into(),
                    });
                }
                Some(element) => serde_json::from_str(&element.text().collect::<String>())
                    .map_err(|error| FetchMetadataError::Setup {
                        stage: "Parse JSON from String".to_owned(),
                        message: error.to_string(),
                    })?,
            };
        let amazon_id = extract_amazon_id(&json, &goodreads_id)?;
        let (title, _subtitle) = extract_title_and_subtitle(&json, &amazon_id)?;
        let image_url = extract_image_url(&json, &amazon_id);
        let contributors = extract_contributors(&json, &amazon_id);
        let publication_date = extract_publication_date(&json, &amazon_id);
        let number_of_pages = extract_page_count(&json, &amazon_id);
        let series = extract_series(&json, &amazon_id);

        Ok(BookMetadata::new(
            &title,
            publication_date,
            contributors,
            series,
            number_of_pages,
            image_url,
            goodreads_id,
        ))
    }
}

/// takes a URL for a Goodreads link (book, author or series) and extracts the entity's Goodreads
/// ID from it
///
/// # Errors
/// This function returns an error when the supplied URL does not match the expected format and
/// when parsing the numeric string to a numeric type fails
fn extract_goodreads_id_from_link(link: &str) -> Result<GoodreadsId, FetchMetadataError> {
    Ok(GoodreadsId::new(
        link.splitn(4, '/')
            .nth(3)
            .unwrap_or("")
            .split('?')
            .next()
            .ok_or_else(|| FetchMetadataError::Extraction {
                key: "Goodreads ID".to_owned(),
                message: "failed to extract Goodreads ID from URL".to_owned(),
            })?
            .chars()
            .take_while(|character| character.is_numeric())
            .collect::<String>()
            .parse::<i64>()
            .map_err(|error| FetchMetadataError::Extraction {
                key: "Goodreads ID".to_owned(),
                message: error.to_string(),
            })?,
    ))
}

/// Helper function to determine if two strings match, ignoring upper and lower case as well as
/// interpunctuations in initials.
fn matches(str1: &str, str2: &str) -> bool {
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

/// Extracts a book's Amazon ID based on its Goodreads ID from the JSON metadata
/// # Errors
/// Fails if the Amazon ID cannot be extracted
fn extract_amazon_id(
    metadata: &Value,
    goodreads_id: &GoodreadsId,
) -> Result<String, FetchMetadataError> {
    let amazon_id_key = format!("getBookByLegacyId({{\"legacyId\":\"{goodreads_id}\"}})");
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let amazon_id =
        &metadata["props"]["pageProps"]["apolloState"]["ROOT_QUERY"][amazon_id_key]["__ref"];
    let Some(amazon_id) = to_string(amazon_id) else {
        error!("Failed to scrape Amazon ID");
        return Err(FetchMetadataError::Extraction {
            key: "Amazon ID".into(),
            message: "failed to extract Amazon ID".into(),
        });
    };

    Ok(amazon_id)
}

/// Extracts title and subtitle out of metadata JSON
/// # Errors
/// Fails if the title cannot be extracted. Missing subtitle is not an error, as not every book has
/// a subtitle.
fn extract_title_and_subtitle(
    metadata: &Value,
    amazon_id: &str,
) -> Result<(String, Option<String>), FetchMetadataError> {
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let title = &metadata["props"]["pageProps"]["apolloState"][amazon_id]["title"];
    let Some(title) = to_string(title) else {
        error!("Failed to scrape book title");
        return Err(FetchMetadataError::Extraction {
            key: "title".into(),
            message: "failed to extract title".into(),
        });
    };

    match title.split_once(':') {
        Some((title, subtitle)) => Ok((title.to_owned(), Some(subtitle.trim().to_owned()))),
        None => Ok((title.clone(), None)),
    }
}

/// Extracts a book's image URL from the metadata JSON. A book may not have an image, so this
/// function returns `Option`
fn extract_image_url(metadata: &Value, amazon_id: &str) -> Option<String> {
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let url = &metadata["props"]["pageProps"]["apolloState"][amazon_id]["imageUrl"];
    to_string(url)
}

/// Extracts all contributors of a book from its metatada JSON and filters out any non-authors.
fn extract_contributors(metadata: &Value, amazon_id: &str) -> Vec<BookContributor> {
    let mut contributors = Vec::new();

    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let primary =
        metadata["props"]["pageProps"]["apolloState"][amazon_id]["primaryContributorEdge"]
            .as_object()
            .map(|obj| (to_string(&obj["role"]), to_string(&obj["node"]["__ref"])));

    match primary {
        Some((Some(role), Some(reference))) => {
            if let Some(contributor) = fetch_contributor(metadata, (role, reference)) {
                contributors.push(contributor);
            }
        }
        Some(_) => {
            warn!("Failed to parse contributor");
        }
        None => (),
    }

    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let Some(secondary) =
        metadata["props"]["pageProps"]["apolloState"][amazon_id]["secondaryContributorEdges"]
            .as_array()
    else {
        return contributors
            .into_iter()
            .filter(|contributor| !contributor.name.to_lowercase().eq("unknown author"))
            .collect();
    };

    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    for contrib in secondary {
        let Some(role) = to_string(&contrib["role"]) else {
            warn!("Failed to parse contributor role");
            continue;
        };
        let Some(key) = to_string(&contrib["node"]["__ref"]) else {
            warn!("Failed to parse contributor key");
            continue;
        };
        // Only keep contributors that are authors
        if role != "Author" {
            info!("Contributor not an author, skipping...");
            continue;
        }

        if let Some(contributor) = fetch_contributor(metadata, (role, key)) {
            contributors.push(contributor);
        }
    }

    contributors
        .into_iter()
        .filter(|contributor| !contributor.name.to_lowercase().eq("unknown author"))
        .collect()
}

/// Parses metadata JSON and extracts all contributors including their name, role and Goodreads ID
fn fetch_contributor(metadata: &Value, (role, key): (String, String)) -> Option<BookContributor> {
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let contributor = &metadata["props"]["pageProps"]["apolloState"][&key]["name"];
    let name = to_string(contributor);
    // First, try to extract Goodreads ID from "legacyId" field
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let goodreads_id = if let Some(id) =
        metadata["props"]["pageProps"]["apolloState"][&key]["legacyId"].as_i64()
    {
        GoodreadsId::new(id)
    } else {
        let url = metadata["props"]["pageProps"]["apolloState"][&key]["webUrl"].as_str()?;
        extract_goodreads_id_from_link(url).ok()?
    };

    if name.is_none() {
        warn!("Failed to parse contributor");
    }

    name.map(|n| BookContributor::new(&n, &role, goodreads_id))
}

/// Extracts a book's publication date from its metadata JSON
fn extract_publication_date(metadata: &Value, amazon_id: &str) -> Option<NaiveDateTime> {
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    #[allow(clippy::pattern_type_mismatch, reason = "false positive")]
    if let Value::Number(number) =
        &metadata["props"]["pageProps"]["apolloState"][amazon_id]["details"]["publicationTime"]
    {
        let timestamp = number.as_i64().and_then(DateTime::from_timestamp_millis);

        if timestamp.is_none() {
            warn!("Failed to parse publication date");
        }
        timestamp.map(|date| date.naive_utc())
    } else {
        warn!("No publication date in JSON found!");
        None
    }
}

/// Extracts a book's page count from its metadata JSON
fn extract_page_count(metadata: &Value, amazon_id: &str) -> Option<i64> {
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let count =
        metadata["props"]["pageProps"]["apolloState"][amazon_id]["details"]["numPages"].as_i64();
    match count {
        Some(0) => None,
        val => val,
    }
}

/// Extracts a book's series from its metadata JSON
fn extract_series(metadata: &Value, amazon_id: &str) -> Vec<BookSeries> {
    let empty_vec: Vec<Value> = Vec::new();

    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let series_array = metadata["props"]["pageProps"]["apolloState"][amazon_id]["bookSeries"]
        .as_array()
        .unwrap_or(&empty_vec);
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let series_info = series_array
        .iter()
        .filter_map(|series| {
            let Some(number) = series["userPosition"]
                .as_str()
                .map(|string| string.split('-').next().unwrap_or(""))
                .and_then(|string| string.parse::<f32>().ok())
            else {
                warn!("Failed to parse series number");
                return None;
            };

            let Some(key) = to_string(&series["series"]["__ref"]) else {
                warn!("Failed to parse series key");
                return None;
            };

            let title = &metadata["props"]["pageProps"]["apolloState"][&key]["title"];
            let Some(title) = to_string(title) else {
                warn!("Failed to parse series title");
                return None;
            };

            let web_url = &metadata["props"]["pageProps"]["apolloState"][&key]["webUrl"];
            let Ok(goodreads_id) = extract_goodreads_id_from_link(web_url.as_str().unwrap_or(""))
            else {
                warn!("Failed to parse series ID");
                return None;
            };

            Some(BookSeries::new(&title, number, goodreads_id))
        })
        .collect::<Vec<BookSeries>>();
    series_info
}

impl MetadataProvider {
    /// Create a new HTTP request client, to be used for all subsequent metadata scraping requests
    /// # Errors
    /// Fails in case any of the reqwest `ClientBuilder` methods fail
    #[inline]
    pub fn create() -> Result<Self, FetchMetadataError> {
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
        let title_selector = Selector::parse(r#"a[class="bookTitle"]"#).map_err(|error| {
            FetchMetadataError::Setup {
                stage: "Title CSS Selector".to_owned(),
                message: error.to_string(),
            }
        })?;
        let author_selector = Selector::parse(r#"a[class="authorName"]"#).map_err(|error| {
            FetchMetadataError::Setup {
                stage: "Author CSS Selector".to_owned(),
                message: error.to_string(),
            }
        })?;
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
            .map(|http_client| Self {
                http_client,
                title_selector,
                author_selector,
            })
            .map_err(|error| FetchMetadataError::Setup {
                stage: "Client Creation".to_owned(),
                message: error.to_string(),
            })
    }

    /// Private fetcher function to execute queries on Goodreads
    ///
    /// # Errors
    /// This function fails if the HTTP request cannot be sent, or if parsing the resulting HTTP
    /// response fails
    #[inline]
    async fn fetch_id(&self, query: &str) -> Result<Html, FetchMetadataError> {
        let url = format!("https://www.goodreads.com/search?q={}", encode(query));

        Ok(Html::parse_document(
            &self
                .http_client
                .get(&url)
                .send()
                .await
                .map_err(|error| FetchMetadataError::Request {
                    request_type: "GET".to_owned(),
                    url,
                    message: error.to_string(),
                })?
                .text()
                .await
                .map_err(|error| FetchMetadataError::Extraction {
                    key: "HTTP Response".to_owned(),
                    message: error.to_string(),
                })?,
        ))
    }
}

/// Helper function to easily convert a JSON `Value` into a `String` and replaces all whitespaces
/// with just a single whitespace.
fn to_string(value: &Value) -> Option<String> {
    match Regex::new(r"\s{2,}") {
        Ok(re) => value
            .as_str()
            .map(str::trim)
            .map(|string| re.replace_all(string, " ").to_string())
            .filter(|string| !string.is_empty()),
        Err(error) => {
            warn!("Failed to construct regex for {value}, {error}");
            None
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "Tests")]
#[allow(clippy::unreadable_literal, reason = "Tests")]
mod tests {

    use super::*;
    use chrono::{NaiveDate, NaiveTime};
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn storm_front_id() {
        let fetcher = MetadataProvider::create().unwrap();
        let goodreads_id = fetcher.fetch_id_with_title("Storm Front").await.unwrap();

        assert_eq!(goodreads_id, Some(GoodreadsId::new(47212)));
    }

    #[tokio::test]
    async fn storm_front_metadata() {
        let fetcher = MetadataProvider::create().unwrap();
        let metadata = fetcher
            .fetch_metadata(GoodreadsId::new(47212))
            .await
            .unwrap();

        assert_eq!(metadata.title, "Storm Front".to_owned());
        assert_eq!(
            metadata.contributors,
            vec![BookContributor::new(
                "Jim Butcher",
                "Author",
                GoodreadsId::new(10746)
            )]
        );
        assert_eq!(
            metadata.publication_date,
            Some(NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2000, 4, 1).unwrap(),
                NaiveTime::from_hms_opt(8, 0, 0).unwrap()
            ))
        );
        assert_eq!(
            metadata.series,
            vec![BookSeries::new(
                "The Dresden Files",
                1.0,
                GoodreadsId::new(40346)
            )]
        );
    }
}
