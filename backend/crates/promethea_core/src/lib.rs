//! `promethea_core`
//!
//! Core library for the platform-independent logic of Promethea. This library aims to provide a crate
//! that can be used both by a Tauri application and a HTTP server to avoid implementing the same
//! logic twice.

pub mod database;

pub mod scraper;
