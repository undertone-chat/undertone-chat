use anyhow::{Result, anyhow};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::time::Instant;

pub type SharedConnectionState = Arc<Mutex<ClientConnectionState>>;

pub(super) enum ConnectionLifecycleState {
    Disconnected,
    Connecting,
    Connected(ConnectionHandle),
}

pub(super) struct ConnectionHandle {
    pub(super) shutdown_sender: Option<oneshot::Sender<()>>,
    pub(super) task_handle: JoinHandle<()>,
    pub(super) connected_host: String,
    pub(super) connected_port: u16,
    pub(super) connected_at: Instant,
}

pub struct ClientConnectionState {
    pub(super) lifecycle_state: ConnectionLifecycleState,
    pub(super) last_error_message: Option<String>,
}

impl Default for ClientConnectionState {
    fn default() -> Self {
        Self {
            lifecycle_state: ConnectionLifecycleState::Disconnected,
            last_error_message: None,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ConnectionCommandResult {
    pub(super) is_connected: bool,
    pub(super) message: String,
}

pub(super) fn validate_connect_input(host: &str) -> Result<()> {
    if host.trim().is_empty() {
        return Err(anyhow!("Server host cannot be empty."));
    }

    if host.parse::<std::net::IpAddr>().is_err() && !hostname_validator::is_valid(host) {
        return Err(anyhow!(
            "Server host must be a valid IP address or hostname. got {}",
            host
        ));
    }

    Ok(())
}

pub(super) fn set_disconnected_state(shared_state: &SharedConnectionState) {
    if let Ok(mut guard) = shared_state.lock() {
        guard.lifecycle_state = ConnectionLifecycleState::Disconnected;
    }
}

pub(super) fn set_last_error_message(shared_state: &SharedConnectionState, message: String) {
    if let Ok(mut guard) = shared_state.lock() {
        guard.last_error_message = Some(message);
    }
}
