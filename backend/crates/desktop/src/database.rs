use crate::errors::Error;
use crate::state::{AppState, APP_CONFIG_PATH, LIBRARY_DATABASE_NAME};
use chrono::{DateTime, Utc};
use epub::doc::EpubDoc;
use promethea_core::scraper::request_builder::MetadataRequestBuilder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use tauri::{AppHandle, State};
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DbInitStatus {
    Loaded,
    NeedsSetup { reason: Option<String> },
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct BookRecord {
    book_id: i64,
    title: String,
    sort: String,
    #[sqlx(json)]
    authors: Vec<String>,
    #[sqlx(json)]
    authors_sort: Vec<String>,
    #[sqlx(json)]
    series_and_volume: Vec<SeriesAndVolume>,
    number_of_pages: u32,
    goodreads_id: u64,
    date_added: DateTime<Utc>,
    date_published: DateTime<Utc>,
    date_modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct SeriesAndVolume {
    series: String,
    sort: String,
    volume: f64,
}

#[tauri::command]
pub async fn create_new_db(
    state: State<'_, AppState>,
    app: AppHandle,
    folder: String,
) -> Result<(), Error> {
    let db_file_path = PathBuf::from(folder).join(PathBuf::from(LIBRARY_DATABASE_NAME));
    std::fs::File::create(db_file_path.clone()).unwrap();

    // update config store
    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!("Updated database path in store to {db_file_path:?}");

    state.connect_db_with_path(db_file_path).await?;

    Ok(())
}

#[tauri::command]
pub async fn open_existing_db(
    state: State<'_, AppState>,
    app: AppHandle,
    path: String,
) -> Result<(), Error> {
    let db_file_path = PathBuf::from(path);

    let store = app.store(APP_CONFIG_PATH)?;
    store.set("library-path", json!({ "value": db_file_path.to_str() }));
    log::info!("Updated database path in store to {db_file_path:?}");

    state.connect_db_with_path(db_file_path).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_init_status(state: State<'_, AppState>) -> Result<DbInitStatus, ()> {
    if state.db_pool.read().await.is_some() {
        Ok(DbInitStatus::Loaded)
    } else {
        Ok(DbInitStatus::NeedsSetup {
            reason: state.last_error.read().await.clone(),
        })
    }
}

#[tauri::command]
pub async fn fetch_books(state: State<'_, AppState>) -> Result<Vec<BookRecord>, String> {
    let read_guard = state.db_pool.read().await;
    if let Some(pool) = &*read_guard {
        let query = "
            WITH series_info
                AS (SELECT bsl.book,
                            Json_group_array(Json_object('series', s.NAME, 'sort', s.sort,
                                            'volume',
                                            bsl.entry)) series_and_volume
                    FROM   series AS s
                            JOIN books_series_link bsl
                            ON bsl.series = s.id
                    GROUP  BY bsl.book),
                authors_info
                AS (SELECT Json_group_array(a.NAME) authors,
                            Json_group_array(a.sort) authors_sort,
                            bal.book
                    FROM   authors AS a
                            JOIN books_authors_link bal
                            ON a.id = bal.author
                    GROUP  BY bal.book)
            SELECT id            AS book_id,
                title,
                sort,
                date_added,
                date_published,
                last_modified AS date_modified,
                number_of_pages,
                goodreads_id,
                authors,
                authors_sort,
                CASE
                    WHEN series_and_volume IS NULL
                        OR Trim(series_and_volume) = '' THEN '[]'
                    WHEN Json_valid(series_and_volume) = 1 THEN series_and_volume
                    ELSE '[]'
                END           AS series_and_volume
            FROM   books
                LEFT JOIN series_info
                        ON series_info.book = books.id
                JOIN authors_info
                    ON authors_info.book = books.id
            ORDER  BY books.date_added ASC  ";
        let books: Vec<BookRecord> = sqlx::query_as(query).fetch_all(pool).await.unwrap();
        return Ok(books);
    }

    Err(String::from("Database pool unavailable"))
}

#[tauri::command]
pub async fn add_book(state: State<'_, AppState>, path: PathBuf) -> Result<(), Error> {
    log::info!("Received request to add book from {path:?}");

    // Extract bare minimum metadata (title + author(s)) from EPUB file
    let doc = EpubDoc::new(path).unwrap();
    dbg!(&doc.metadata);

    let title = doc.get_title().unwrap();
    let authors = doc
        .metadata
        .iter()
        .filter(|e| e.property == "creator")
        .map(|e| e.value.clone())
        .collect::<Vec<String>>();

    // Use those title and author(s) to find the appropriate book on Goodreads and scrape it for
    // more data
    let request = MetadataRequestBuilder::default()
        .with_title(&title)
        .with_author(authors.first().unwrap());

    match request.execute().await.unwrap() {
        Some(metadata) => {
            dbg!(metadata);
        }
        None => log::info!("No metadata found for this book"),
    }

    // Assemble data into SQL query

    // Basic logic: Upsert new book title, author(s) name(s) and series title(s), meaning try to
    // insert and then fetch resulting ID, do not insert if already present and fetch previously
    // existing ID.

    // For sorting, define helper functions for common stuff like titles starting with "The", "A",
    // "An", and for authors try "Lastname, Firstname"

    Ok(())
}
