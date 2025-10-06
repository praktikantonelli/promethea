use serde_json::json;
use std::path::PathBuf;
use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to open key-value config")]
    StoreAccess(#[from] tauri_plugin_store::Error),
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn set_library_path(app: AppHandle<Wry>, path: String) -> Result<(), Error> {
    let path = PathBuf::from(path);
    let store = app.store("promethea-config.json")?;
    store.set("library-path", json!({"value": path}));

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default().plugin(tauri_plugin_store::Builder::new().build());
    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let store = app.store("promethea-config.json")?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
