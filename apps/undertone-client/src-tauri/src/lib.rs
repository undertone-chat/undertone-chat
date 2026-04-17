mod commands;
pub mod error;
pub mod session;

use serde::Serialize;
use session::SessionState;
use std::sync::Mutex;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LogMessage {
    message: String,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SessionState::default()))
        .invoke_handler(tauri::generate_handler![commands::ui::test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
