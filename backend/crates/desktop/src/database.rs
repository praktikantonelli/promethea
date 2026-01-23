use crate::errors::Error;
use crate::state::{APP_CONFIG_PATH, AppState, LIBRARY_DATABASE_NAME};
use chrono::Local;
use core::future::Future;
use core::iter::zip;
use epub::doc::EpubDoc;
use futures::future::join_all;
use promethea_core::database::types::{AuthorRecord, BookRecord, SeriesAndVolumeRecord};
use promethea_core::scraper::request_builder::MetadataRequestBuilder;
use promethea_core::scraper::sorting::{get_name_sort, get_title_sort};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::time::Instant;
use tauri::{AppHandle, Emitter as _, State};
use tauri_plugin_store::StoreExt as _;
use tokio::task;
use tracing::{Instrument as _, info_span, instrument};

async fn resolve_sort_with_fallback<Primary, PrimaryFut, Fallback, E>(
    primary: Primary,
    fallback: Fallback,
) -> String
where
    Primary: FnOnce() -> PrimaryFut,
    PrimaryFut: Future<Output = Result<Option<String>, E>>,
    Fallback: FnOnce() -> String,
{
    match primary().await {
        Ok(Some(val)) => val,
        _ => fallback(),
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum DbInitStatus {
    Loaded,
    NeedsSetup { reason: Option<String> },
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
    log::info!(
        "Updated database path in store to {}",
        db_file_path.display()
    );

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
    log::info!(
        "Updated database path in store to {}",
        db_file_path.display()
    );

    state.connect_db_with_path(db_file_path).await?;

    Ok(())
}

#[tauri::command]
pub async fn get_init_status(state: State<'_, AppState>) -> Result<DbInitStatus, ()> {
    if state.db.read().await.is_some() {
        Ok(DbInitStatus::Loaded)
    } else {
        Ok(DbInitStatus::NeedsSetup {
            reason: state.last_error.read().await.clone(),
        })
    }
}

#[tauri::command]
pub async fn fetch_books(state: State<'_, AppState>) -> Result<Vec<BookRecord>, String> {
    let read_guard = state.db.read().await;
    if let Some(db) = &*read_guard {
        let books = db.fetch_books_query();
        return books
            .await
            .map_err(|err| format!("Failed to run query: {err}"));
    }

    Err(String::from("Database pool unavailable"))
}

#[instrument(
    name = "cmd.add_book",
    skip(app, state),
    fields(path = ?path)
)]
#[tauri::command]
pub async fn add_book(
    app: AppHandle,
    state: State<'_, AppState>,
    path: PathBuf,
) -> Result<(), Error> {
    tracing::info!("Received request to add book from {path:?}");

    // Phase 1: Extract title + author(s) from EPUB file
    let parse_span = info_span!("epub.parse");
    let (title, authors) = task::spawn_blocking({
        let path = path.clone();
        move || {
            let _e = parse_span.enter();

            let t0 = Instant::now();

            // Extract bare minimum metadata (title + author(s)) from EPUB file
            let doc = EpubDoc::new(path).unwrap();

            let title = doc.get_title().unwrap();
            let authors = doc
                .metadata
                .iter()
                .filter(|item| item.property == "creator")
                .map(|item| item.value.clone())
                .collect::<Vec<String>>();

            tracing::info!(
                elapsed_ms = t0.elapsed().as_millis(),
                author_count = authors.len(),
                "epub metadata extracted"
            );
            Ok::<_, Error>((title, authors))
        }
    })
    .await
    .map_err(|err| Error::Other(err.to_string()))??;

    // Phase 2: Use found title and author(s) to scrape Goodreads for metadata
    let first_author = authors.first().cloned().unwrap_or_default();
    let scrape_span = info_span!("metadata.scrape", title = %title, author = %first_author);
    let metadata = async {
        let t0 = Instant::now();

        let request = MetadataRequestBuilder::default()
            .with_title(&title)
            .with_author(&first_author);

        let result = request
            .execute()
            .await
            .map_err(|err| Error::Other(format!("{err:?}")))?;

        tracing::info!(
            elapsed_ms = t0.elapsed().as_millis(),
            found = result.is_some(),
            "metadata scraping done"
        );
        Ok::<_, Error>(result)
    }
    .instrument(scrape_span)
    .await?;

    let Some(metadata) = metadata else {
        tracing::info!("no metadat found");
        return Err(Error::Other(
            "Failed to find metadata for given book".to_string(),
        ));
    };

    let read_guard = state.db.read().await;
    let Some(db) = &*read_guard else {
        tracing::warn!("Database currently not available");
        return Err(Error::Other(
            "Failed to get database connection from app state".to_string(),
        ));
    };

    let title_sort = get_title_sort(&title);
    let authors = metadata.contributors;
    let authors_sort = join_all(authors.iter().map(|key| async move {
        resolve_sort_with_fallback(
            || db.try_fetch_author_sort(&key.name),
            || get_name_sort(&key.name),
        )
        .await
    }))
    .await;
    let authors = zip(authors, authors_sort)
        .map(|(author, author_sort)| {
            AuthorRecord::new(
                author.name,
                author_sort,
                author.goodreads_id.parse().unwrap(),
            )
        })
        .collect::<Vec<AuthorRecord>>();

    // Series sort string(s) => Same as authors
    let series = metadata.series;
    let series_sort = join_all(series.iter().map(|key| async move {
        resolve_sort_with_fallback(
            || db.try_fetch_series_sort(&key.title),
            || get_title_sort(&key.title),
        )
        .await
    }))
    .await;
    let series_and_volume = zip(series, series_sort)
        .map(|(series, series_sort)| {
            SeriesAndVolumeRecord::new(
                series.title,
                series_sort,
                series.number as f64,
                series.goodreads_id.parse().unwrap(),
            )
        })
        .collect::<Vec<SeriesAndVolumeRecord>>();
    // Date added => get today's date
    let date_added = Local::now().to_utc();
    // Date updated => get today's date
    let date_updated = date_added;

    // Assemble data into SQL query
    let book_record = BookRecord::new(
        -1,
        title,
        title_sort,
        authors,
        series_and_volume,
        metadata.page_count.unwrap(),
        metadata.goodreads_id.unwrap().parse().unwrap(),
        date_added.naive_utc(),
        metadata.publication_date.unwrap().naive_utc(),
        date_updated.naive_utc(),
    );

    let read_guard = state.db.read().await;
    let db = match &*read_guard {
        Some(db) => db,
        None => {
            tracing::error!("Could not get DB read guard!");
            return Err(Error::Other("Could not get DB read guard".into()));
        }
    };
    if let Err(err) = db.insert_book(&book_record).await {
        tracing::error!("Failed to add book: {err}");
        return Err(Error::Other(err.to_string()));
    }

    tracing::info!("Successfully added book");
    app.emit("db:changed", ())?;
    Ok(())
}
