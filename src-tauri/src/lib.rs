use tauri::{AppHandle, Wry};
use tauri_plugin_store::StoreExt;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Failed to open key-value config")]
    StoreAccess(#[from] tauri_plugin_store::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    log::warn!("Received request: greet()");
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tauri::command]
fn notify_library_path_set(app: AppHandle<Wry>) -> Result<(), Error> {
    log::info!("Received request: notify_library_path_set()");
    let store = app.store("promethea-config.json")?;
    let path = store.get("library_path");
    dbg!(path);

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build());
    #[cfg(debug_assertions)]
    {
        let devtools = tauri_plugin_devtools::init();
        builder = builder.plugin(devtools);
    }

    builder
        .invoke_handler(tauri::generate_handler![greet, notify_library_path_set])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
