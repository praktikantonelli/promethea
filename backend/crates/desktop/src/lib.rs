//! `desktop`
//!
//! This crate contains everything Tauri-specific for promethea
use crate::database::{add_book, create_new_db, fetch_books, get_init_status, open_existing_db};
use crate::state::{APP_CONFIG_PATH, AppState};
use anyhow::Error;
use std::path::PathBuf;
use tauri::Manager as _;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_store::StoreExt as _;
#[cfg(not(debug_assertions))]
use tracing_subscriber::{EnvFilter, fmt};
/// Database module, holds everything dealing with accessing the database from the Tauri
/// application
mod database;
/// Error types
mod errors;
/// App state management
mod state;
use std::env;
use tauri::async_runtime;

#[allow(
    clippy::missing_inline_in_public_items,
    reason = "Executed once per run, never across crate boundaries"
)]
#[allow(
    clippy::print_stderr,
    reason = "Tracing might not be available here if run_safe() failed before its initialization"
)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(error) = run_safe() {
        eprintln!("Failed to start Promethea! Error: {error}");
    }
}

/// Encapsulated run function that allows returning errors instead of always panicking on `Err` or
/// `None` variants. Note that, since `run()` is the entry point for mobile, it has to keep its
/// signature of not returning anything.
#[allow(clippy::exit, reason = "Happens in Tauri macro, cannot be avoided")]
fn run_safe() -> Result<(), Error> {
    #[cfg(not(debug_assertions))]
    {
        let subscriber = fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("Unable to set global tracing subscriber");
    }
    let enable_devtools = env::var("ENABLE_DEVTOOLS")
        .map(|val| val == "true" || val == "1")
        .unwrap_or(false);
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init());
    builder
        .manage(AppState::new())
        .setup(move |app| {
            // Let app manage SQLite database state
            let (tauri_plugin_log, max_level, logger) = tauri_plugin_log::Builder::default()
                .with_colors(ColoredLevelConfig::default())
                .level(log::LevelFilter::Info)
                .level_for("promethea", log::LevelFilter::Info)
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Webview,
                ))
                .split(app.handle())?;

            #[cfg(debug_assertions)]
            {
                if enable_devtools {
                    // With debug assertions, use CrabNebula dev tools plugin
                    let mut devtools_builder = tauri_plugin_devtools::Builder::default();
                    devtools_builder.attach_logger(logger);
                    app.handle().plugin(devtools_builder.init())?;
                } else {
                    tauri_plugin_log::attach_logger(max_level, logger)?;
                }
            }
            #[cfg(not(debug_assertions))]
            {
                // Without debug assertions, use regular logger plugin
                tauri_plugin_log::attach_logger(max_level, logger)?;
            }
            app.handle().plugin(tauri_plugin_log)?;

            let store = app.store(APP_CONFIG_PATH)?;
            if let Some(db_path) = store.get("library-path") {
                log::info!("Using database at {db_path:?}");
                let app_state = app.state::<AppState>().clone();
                async_runtime::block_on(async move {
                    let path = PathBuf::from(
                        db_path
                            .get("value")
                            .unwrap_or(&serde_json::Value::Null)
                            .as_str()
                            .unwrap_or(""),
                    );
                    if let Err(err) = app_state.connect_db_with_path(path).await {
                        log::error!("DB init on startup failed: {err}");
                    } else {
                        log::info!("DB connected successfully");
                    }
                });
            } else {
                log::info!("No database path in config, wait for user to provide one");
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_new_db,
            open_existing_db,
            get_init_status,
            fetch_books,
            add_book
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
