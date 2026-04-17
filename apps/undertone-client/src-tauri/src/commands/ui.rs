use tauri::{AppHandle, Emitter};

use crate::LogMessage;

#[tauri::command]
pub fn test(app: AppHandle) {
    tracing::debug!("Got test command.");
    app.emit(
        "log-message",
        LogMessage {
            message: "Im a log! poop!".to_string(),
        },
    )
    .unwrap();
}
