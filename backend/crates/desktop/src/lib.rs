use crate::database::{add_book, create_new_db, fetch_books, get_init_status, open_existing_db};
use crate::state::{AppState, APP_CONFIG_PATH};
use std::path::PathBuf;
use tauri::Manager;
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tauri_plugin_store::StoreExt;

mod database;
mod errors;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init());
    builder
        .manage(AppState::new())
        .setup(|app| {
            // Let app manage SQLite database state
            let (tauri_plugin_log, max_level, logger) = tauri_plugin_log::Builder::default()
                .with_colors(ColoredLevelConfig::default())
                .level(log::LevelFilter::Info)
                .level_for("promethea", log::LevelFilter::Trace)
                .split(app.handle())?;

            if cfg!(debug_assertions) {
                // With debug assertions, use CrabNebula dev tools plugin
                let mut devtools_builder = tauri_plugin_devtools::Builder::default();
                devtools_builder.attach_logger(logger);
                app.handle().plugin(devtools_builder.init())?;
            } else {
                // Without debug assertions, use regular logger plugin
                tauri_plugin_log::attach_logger(max_level, logger)?;
            }
            app.handle().plugin(tauri_plugin_log)?;

            let store = app.store(APP_CONFIG_PATH).unwrap();
            if let Some(db_path) = store.get("library-path") {
                log::info!("Using database at {db_path:?}");
                let app_state = app.state::<AppState>().clone();
                tauri::async_runtime::block_on(async move {
                    let path = PathBuf::from(db_path.get("value").unwrap().as_str().unwrap());
                    if let Err(err) = app_state.connect_db_with_path(path).await {
                        log::error!("DB init on startup failed: {err}");
                    } else {
                        log::info!("DB connected successfully");
                    }
                })
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
