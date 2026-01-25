use crate::scraper::errors::ScraperError;
use chrono::{DateTime, Utc};
use log::{error, info, warn};
use regex::Regex;
use reqwest::get;
use scraper::{Html, Selector};
use serde_json::Value;

/// The primary data structure containing the metadata of a book.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct BookMetadata {
    /// The main title of the book.
    pub title: String,
    /// The publication date of the book, represented as a UTC datetime.
    pub publication_date: Option<DateTime<Utc>>,
    /// A list of contributors to the book, each represented as a `BookContributor`.
    pub contributors: Vec<BookContributor>,
    /// A list of series information, if the book is part of a series, represented as a `BookSeries`.
    pub series: Vec<BookSeries>,
    /// The number of pages in the book, if available.
    pub page_count: Option<i64>,
    /// A URL to an image of the book's cover, if available.
    pub image_url: Option<String>,
    /// The ID with which the book's metadata has been fetched
    pub goodreads_id: Option<String>,
}

/// Represents an individual who contributed to the book, such as an author or editor.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct BookContributor {
    /// The name of the contributor.
    pub name: String,
    /// The role of the contributor, such as "Author" or "Illustrator".
    pub role: String,
    /// The Goodreads ID of the contributor
    pub goodreads_id: String,
}

/// Represents series information for a book, including the series title and book's position within the series.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub struct BookSeries {
    /// The title of the series.
    pub title: String,
    /// The position of the book within the series, represented as a float to accommodate cases like "1.5".
    pub number: f32,
    /// The Goodreads ID of the series
    pub goodreads_id: String,
}

/// Fetches all metadata of a book given its Goodreads ID
/// # Errors
/// This function fails if fetching the JSON data from the scraper fails or if the Amazon ID cannot
/// be extracted.
#[allow(
    clippy::missing_inline_in_public_items,
    reason = "Called rarely, large function"
)]
pub async fn fetch_metadata(goodreads_id: &str) -> Result<BookMetadata, ScraperError> {
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

/// Takes a Goodreads ID and extracts the metadata JSON from its corresponding website
/// # Errors
/// This function fails if the HTTP request fails or if parsing the extracting the JSON from the
/// HTML page fails
async fn extract_book_metadata(goodreads_id: &str) -> Result<Value, ScraperError> {
    let url = format!("https://www.goodreads.com/book/show/{goodreads_id}");
    let document = Html::parse_document(&get(&url).await?.text().await?);
    let metadata_selector = Selector::parse(r#"script[id="__NEXT_DATA__"]"#)?;
    let metadata = &document.select(&metadata_selector).next();

    let metadata = match *metadata {
        None => {
            error!("Failed to scrape book metadata");
            return Err(ScraperError::ScrapeError(
                "Failed to scrape book metadata".to_owned(),
            ));
        }
        Some(element) => serde_json::from_str(&element.text().collect::<String>())?,
    };

    Ok(metadata)
}

/// Extracts a book's Amazon ID based on its Goodreads ID from the JSON metadata
/// # Errors
/// Fails if the Amazon ID cannot be extracted
fn extract_amazon_id(metadata: &Value, goodreads_id: &str) -> Result<String, ScraperError> {
    let amazon_id_key = format!("getBookByLegacyId({{\"legacyId\":\"{goodreads_id}\"}})");
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let amazon_id =
        &metadata["props"]["pageProps"]["apolloState"]["ROOT_QUERY"][amazon_id_key]["__ref"];
    let Some(amazon_id) = to_string(amazon_id) else {
        error!("Failed to scrape Amazon ID");
        return Err(ScraperError::ScrapeError(
            "Failed to scrape Amazon ID".to_owned(),
        ));
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
) -> Result<(String, Option<String>), ScraperError> {
    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let title = &metadata["props"]["pageProps"]["apolloState"][amazon_id]["title"];
    let Some(title) = to_string(title) else {
        error!("Failed to scrape book title");
        return Err(ScraperError::ScrapeError(
            "Failed to scrape book title".to_owned(),
        ));
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

/// Extracts all contributors of a book from its metatada JSON and filters out any non-authors. A
/// book may have more than one author, so this function returns a vector. In case of problems, the
/// function returns an empty vector.
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

/// Extracts a book's publication date from its metadata JSON
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

/// Extracts a book's series from its metadata JSON. Because a book may belong to multiple series
/// (or one series and one overarching universe etc.), this function returns a vector. A book with
/// no series returns an empty vector.
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

/// Helper function to extract the Goodreads ID from a URL.
fn extract_id_from_url(url: &Value) -> Option<String> {
    let url = url.as_str()?;
    let replaced = url.replace("https://www.goodreads.com/series/", "");
    let id_raw = replaced.split('-').next()?;
    let id = String::from(id_raw);
    Some(id)
}

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests are constructed with known values"
)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn fetch_metadata_test() {
        let expected_series = vec![
            BookSeries {
                title: "Percy Jackson and the Olympians".to_owned(),
                number: 5.0,
                goodreads_id: "40736".to_owned(),
            },
            BookSeries {
                title: "Camp Half-Blood Chronicles".to_owned(),
                number: 5.0,
                goodreads_id: "183923".to_owned(),
            },
            BookSeries {
                title: "Coleccionable Percy Jackson".to_owned(),
                number: 5.0,
                goodreads_id: "399169".to_owned(),
            },
        ];
        let expected_contributors = vec![BookContributor {
            name: "Rick Riordan".to_owned(),
            role: "Author".to_owned(),
            goodreads_id: "15872".to_owned(),
        }];
        let expected_metadata = BookMetadata {
            title: "The Last Olympian".to_owned(),
            publication_date: Some(DateTime::parse_from_rfc3339("2009-05-05T07:00:00Z").unwrap().to_utc()),
            contributors: expected_contributors,
            series: expected_series,
            page_count: Some(381),
            image_url: Some("https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1723393514i/4556058.jpg".to_owned()),
            goodreads_id: Some(String::from("4556058")),

        };

        let metadata = fetch_metadata("4556058").await.unwrap();
        assert_eq!(metadata, expected_metadata);
    }

    #[tokio::test]
    async fn fetch_metadata_contributor_with_no_legacy_id() {
        let expected_contributor = BookContributor {
            name: "Mark Zug".to_owned(),
            role: "Illustrations".to_owned(),
            goodreads_id: "619712".to_owned(),
        };
        let metadata = extract_book_metadata("7355137").await.unwrap();
        let contributor = fetch_contributor(
            &metadata,
            (
                "Illustrations".to_owned(),
                "Contributor:kca://author/amzn1.gr.author.v1.pVNrjuvKvXYyslKm1pwHxQ".to_owned(),
            ),
        )
        .unwrap();
        assert_eq!(expected_contributor, contributor);
    }

    #[tokio::test]
    async fn multiple_contributors_single_author() {
        // Darke - Angie Sage, Mark Zug(Illustrator)
        let expected_authors = vec![BookContributor {
            name: "Angie Sage".to_owned(),
            role: "Author".to_owned(),
            goodreads_id: "157663".to_owned(),
        }];
        let metadata = fetch_metadata("7355137").await.unwrap();
        assert_eq!(expected_authors, metadata.contributors);
    }

    #[tokio::test]
    async fn multiple_contributors_multiple_authors() {
        // A Memory of Light - Robert Jordan, Brandon Sanderson
        let expected_authors = [
            BookContributor {
                name: "Brandon Sanderson".to_owned(),
                role: "Author".to_owned(),
                goodreads_id: "38550".to_owned(),
            },
            BookContributor {
                name: "Robert Jordan".to_owned(),
                role: "Author".to_owned(),
                goodreads_id: "6252".to_owned(),
            },
        ];
        let contributors = fetch_metadata("7743175").await.unwrap().contributors;
        assert_eq!(contributors.len(), expected_authors.len());
        for contributor in contributors {
            assert!(expected_authors.contains(&contributor));
        }
    }
}
