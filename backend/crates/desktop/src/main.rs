//! The main function for the Promethea desktop application
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(clippy::print_stderr, reason = "No other tracing loaded at this point")]
fn main() {
    if dotenvy::dotenv().is_ok() {
        promethea_lib::run();
    } else {
        eprintln!("Failed to load environment variables!");
    }
}
