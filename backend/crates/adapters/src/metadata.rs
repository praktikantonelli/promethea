use async_trait::async_trait;
use core::time::Duration;
use log::{error, info, warn};
use reqwest::{ClientBuilder, header, redirect::Policy};
use scraper::{Html, Selector};
use serde_json::Value;
use shared_core::domain::metadata::BookMetadata;
use shared_core::domain::metadata::{BookContributor, BookSeries};
use shared_core::domain::repository::GoodreadsId;
use shared_core::ports::metadata::{FetchMetadataError, MetadataProviderPort};
use urlencoding::encode;

pub struct MetadataProvider {
    http_client: reqwest::Client,
}

#[async_trait]
impl MetadataProviderPort for MetadataProvider {
    async fn fetch_goodreads_id(
        &self,
        title: &str,
        author: &str,
    ) -> Result<Option<GoodreadsId>, FetchMetadataError> {
        let query = format!("{} {}", title, author);
        let url = format!("https://www.goodreads.com/search?q={}", encode(&query));

        let document =
            Html::parse_document(&self.http_client.get(&url).send().await?.text().await?);
        let title_selector = Selector::parse(r#"a[class="bookTitle"]"#)?;
        let author_selector = Selector::parse(r#"a[class="authorName"]"#)?;

        for (title_element, author_element) in document
            .select(&title_selector)
            .zip(document.select(&author_selector))
        {
            let found_title = title_element.text().collect::<String>();
            let found_author = author_element.text().collect::<String>();
            let found_link = title_element.value().attr("href")?;
            let found_id = extract_goodreads_id_from_link(found_link)?;

            if matches(&found_title, &title) && matches(&found_author, &author) {
                return Ok(Some(found_id));
            }
        }
        Ok(None)
    }

    async fn fetch_metadata(
        &self,
        goodreads_id: GoodreadsId,
    ) -> Result<BookMetadata, FetchMetadataError> {
        let url = format!("https://www.goodreads.com/book/show/{goodreads_id}");
        let document =
            Html::parse_document(&self.http_client.get(&url).send().await?.text().await?);
        let json_selector = Selector::parse(r#"script[id="__NEXT_DATA__"]"#)?;
        let json = &document.select(&json_selector).next();

        let json = match *json {
            None => {
                return Err(FetchMetadataError::Extraction {
                    key: "JSON".into(),
                    message: "no metadata JSON in Goodreads page".into(),
                });
            }
            Some(element) => serde_json::from_str(&element.text().collect::<String>())?,
        };
        let amazon_id = extract_amazon_id(&json, &goodreads_id);
        let (title, subtitle) = extract_title_and_subtitle(&json, &amazon_id);
        let image_url = extract_image_url(&json, &amazon_id);
        let contributors = extract_contributors(&json, &amazon_id);
        let publication_date = extract_publication_date(&json, &amazon_id);
        let number_of_pages = extract_page_count(&json, &amazon_id);
        let series = extract_series(&json, &amazon_id);

        Ok(BookMetadata::new(
            title,
            publication_date,
            contributors,
            series,
            number_of_pages,
            image_url,
            goodreads_id,
        ))
    }
}

fn extract_goodreads_id_from_link(link: &str) -> Result<GoodreadsId, FetchMetadataError> {
    link.splitn(4, '/')
        .nth(3)
        .unwrap_or("")
        .split('?')
        .next()?
        .chars()
        .take_while(|character| character.is_numeric())
        .collect::<String>()
}

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

fn extract_image_url(metadata: &Value, amazon_id: &str) -> Option<String> {
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let url = &metadata["props"]["pageProps"]["apolloState"][amazon_id]["imageUrl"];
    to_string(url)
}

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
    let Some(goodreads_id) = metadata["props"]["pageProps"]["apolloState"][&key]["legacyId"]
        .as_i64()
        .map(|x| x.to_string())
        .or_else(|| {
            let id = metadata["props"]["pageProps"]["apolloState"][&key]["webUrl"].as_str()?;
            id.strip_prefix("https://www.goodreads.com/author/show/")
                .and_then(|rest| rest.split('.').next())
                .map(str::to_owned)
        })
    else {
        warn!("Failed to parse Goodreads ID");
        return None;
    };

    if name.is_none() {
        warn!("Failed to parse contributor");
    }

    name.map(|n| BookContributor {
        name: n,
        role,
        goodreads_id,
    })
}

fn extract_publication_date(metadata: &Value, amazon_id: &str) -> Option<DateTime<Utc>> {
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
        timestamp
    } else {
        warn!("No publication date in JSON found!");
        None
    }
}

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
            let Some(goodreads_id) = extract_id_from_url(web_url) else {
                warn!("Failed to parse series ID");
                return None;
            };

            Some(BookSeries {
                title,
                number,
                goodreads_id,
            })
        })
        .collect::<Vec<BookSeries>>();
    series_info
}

impl MetadataProvider {
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
}

#[derive(thiserror::Error, Debug)]
enum SearchError {}

#[derive(thiserror::Error, Debug)]
enum ExtractError {}
