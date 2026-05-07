//! `desktop`
//!
//! This crate contains everything Tauri-specific for promethea
use crate::database::{add_book, fetch_books, get_init_status, open_db};
use errors::PrometheaError;
use state::{APP_CONFIG_PATH, AppState, BackendState, RuntimeConfig, build_services};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Manager as _;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_store::StoreExt as _;
use tokio::sync::RwLock;
/// Database module, holds everything dealing with accessing the database from the Tauri
/// application
mod database;
/// Error types
mod errors;
/// New app state management
mod state;
/// App state management
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
fn run_safe() -> Result<(), PrometheaError> {
    let enable_devtools = env::var("ENABLE_DEVTOOLS")
        .map(|val| val == "true" || val == "1")
        .unwrap_or(false);
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init());
    builder
        .setup(move |app| {
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
            let maybe_path = store.get("library-path").and_then(|value| {
                value
                    .get("value")
                    .unwrap_or(&serde_json::Value::Null)
                    .as_str()
                    .map(PathBuf::from)
            });
            let backend = maybe_path.clone().map_or_else(
                || BackendState::NeedsSetup,
                |path| {
                    async_runtime::block_on(build_services(path))
                        .map_or_else(|_| BackendState::NeedsSetup, BackendState::Ready)
                },
            );
            app.manage(AppState {
                config: Arc::new(RwLock::new(RuntimeConfig {
                    library_path: maybe_path,
                })),
                backend: Arc::new(RwLock::new(backend)),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_init_status,
            fetch_books,
            add_book,
            open_db
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}
