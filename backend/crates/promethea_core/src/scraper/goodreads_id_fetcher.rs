use crate::scraper::errors::ScraperError;
use log::warn;
use reqwest::get;
use scraper::{Html, Selector};
use serde_json::Value;
use urlencoding::encode;

#[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
pub async fn verify_id_exists(id: &str) -> bool {
    let url = format!("https://www.goodreads.com/book/show/{id}");
    match get(&url).await {
        Ok(response) => response.status().is_success(),
        Err(error) => {
            warn!("Failed to fetch book page for id {id}: {error}");
            false
        }
    }
}

/// Given ISBN, fetches Goodreads ID
/// # Errors
/// The function fails if the search for the book fails.
#[allow(
    clippy::missing_inline_in_public_items,
    reason = "Called rarely, large function"
)]
pub async fn fetch_id_from_isbn(isbn: &str) -> Result<Option<String>, ScraperError> {
    let url = format!("https://www.goodreads.com/search?q={}", encode(isbn));
    let document = Html::parse_document(&get(&url).await?.text().await?);

    let metadata_selector = Selector::parse(r#"script[id="__NEXT_DATA__"]"#)?;

    let metadata = match document.select(&metadata_selector).next() {
        Some(metadata) => &metadata.text().collect::<String>(),
        None => return Ok(None),
    };

    let metadata: Value = serde_json::from_str(metadata)?;

    #[allow(
        clippy::indexing_slicing,
        reason = "`serde_json::Value` indexing never panics"
    )]
    let goodreads_id = metadata["props"]["pageProps"]["params"]["book_id"]
        .as_str()
        .ok_or(ScraperError::ParseError(
            "Failed to extract Goodreads ID from ISBN search results".to_owned(),
        ))?;

    let goodreads_id = goodreads_id
        .chars()
        .take_while(|character| character.is_numeric())
        .collect::<String>();

    Ok(Some(goodreads_id))
}

/// Given title, fetches Goodreads ID
/// # Errors
/// The function fails if the search for the book fails.
#[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
pub async fn fetch_id_from_title(title: &str) -> Result<Option<String>, ScraperError> {
    let results = search_books(title).await?;

    for (found_title, _, found_id) in results {
        if matches(&found_title, title) {
            return Ok(Some(found_id));
        }
    }

    Ok(None)
}

/// Given title and author, fetches Goodreads ID
/// # Errors
/// The function fails if the search for the book fails.
#[allow(clippy::missing_inline_in_public_items, reason = "Called rarely")]
pub async fn fetch_id_from_title_and_author(
    title: &str,
    author: &str,
) -> Result<Option<String>, ScraperError> {
    let results_with_title = search_books(title).await?;

    for (found_title, found_author, found_id) in results_with_title {
        if matches(&found_title, title) && matches(&found_author, author) {
            return Ok(Some(found_id));
        }
    }

    let results_with_title_author = search_books(&format!("{title} {author}")).await?;

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
async fn search_books(query: &str) -> Result<Vec<(String, String, String)>, ScraperError> {
    let url = format!("https://www.goodreads.com/search?q={}", encode(query));

    let document = Html::parse_document(&get(&url).await?.text().await?);
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

/// Tries and extracts the Goodreads ID out of a Goodreads URL
fn extract_goodreads_id(url: &str) -> String {
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

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    reason = "Tests are predefined and guaranteed to be Some/Ok"
)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_id_from_title_test() {
        let book_title = "The Last Magician";
        assert_eq!(
            fetch_id_from_title(book_title).await.unwrap(),
            Some("30312855".to_owned())
        );
    }

    #[tokio::test]
    async fn fetch_id_from_title_not_found_test() {
        let book_title = "thistitledoesnotexist";
        assert_eq!(fetch_id_from_title(book_title).await.unwrap(), None);
    }

    #[tokio::test]
    async fn fetch_id_from_title_and_author_test() {
        let book_title = "Fire";
        let book_author = "Kristin Cashore";
        assert_eq!(
            fetch_id_from_title_and_author(book_title, book_author)
                .await
                .unwrap(),
            Some("6137154".to_owned())
        );
    }

    #[tokio::test]
    async fn fetch_id_from_title_and_author_not_found_test() {
        let book_title = "thistitledoesnotexist";
        let book_author = "noauthor";
        assert_eq!(
            fetch_id_from_title_and_author(book_title, book_author)
                .await
                .unwrap(),
            None
        );
    }

    #[tokio::test]
    async fn fetch_id_from_isbn_test() {
        let isbn = "9780063021426";
        assert_eq!(
            fetch_id_from_isbn(isbn).await.unwrap(),
            Some("57945316".to_owned())
        );
    }

    #[tokio::test]
    async fn fetch_id_from_isbn_not_found_test() {
        let isbn = "1234001592323";
        assert_eq!(fetch_id_from_isbn(isbn).await.unwrap(), None);
    }

    #[tokio::test]
    async fn verify_id_exists_test() {
        let id = "57945316";
        assert!(verify_id_exists(id).await);
    }

    #[tokio::test]
    async fn verify_id_not_found_test() {
        let id = "bad_id";
        assert!(!(verify_id_exists(id).await));
    }
}
