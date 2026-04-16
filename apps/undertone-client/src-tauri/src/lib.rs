mod control_connection;
use std::sync::{Arc, Mutex};

use control_connection::{ClientConnectionState, SharedConnectionState};
use tokio::runtime::Runtime as TokioRuntime;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn connect() {
    tracing::debug!("Attempting to connect...");
    // Open connection?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let runtime = TokioRuntime::new().expect("failed to create tokio runtime");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(runtime))
        .manage(Arc::new(Mutex::new(ClientConnectionState::default())))
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
