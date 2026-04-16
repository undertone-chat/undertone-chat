use super::config::*;
use super::state::{
    ConnectionCommandResult, ConnectionHandle, ConnectionLifecycleState, SharedConnectionState,
    set_disconnected_state, validate_connect_input,
};

use super::task::run_connection_task;
use anyhow::{Context, Result, anyhow};
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::runtime::Runtime as TokioRuntime;
use tokio::sync::oneshot;
use tokio::time::{Duration, Instant, timeout};

pub(super) fn connect_internal(
    host: String,
    port: u16,
    connection_state: tauri::State<'_, SharedConnectionState>,
    runtime: tauri::State<'_, Mutex<TokioRuntime>>,
) -> Result<ConnectionCommandResult> {
    validate_connect_input(&host).context("invalid connection input")?;

    let mut state_guard = connection_state
        .lock()
        .map_err(|_| anyhow!("connection state mutex poisoned"))
        .context("failed to acquire connection state lock")?;

    match state_guard.lifecycle_state {
        ConnectionLifecycleState::Disconnected => {
            state_guard.lifecycle_state = ConnectionLifecycleState::Connecting
        }
        ConnectionLifecycleState::Connecting => {
            return Err(anyhow!("Client is already connecting").context("invalid connection state"));
        }
        ConnectionLifecycleState::Connected(_) => {
            return Err(anyhow!("Client already connected!"));
        }
    }

    drop(state_guard);

    let address = format!("{}:{}", host, port);

    let runtime_guard = runtime
        .lock()
        .map_err(|_| anyhow!("Failed to lock runtime"))?;

    let connect_result = runtime_guard.block_on(async {
        tokio::time::timeout(
            Duration::from_secs(CONNECT_TIMEOUT_SECONDS),
            TcpStream::connect(&address),
        )
        .await
        .with_context(|| format!("tcp connection to {} failed", address))
    });

    let stream = match connect_result {
        Ok(stream) => stream,
        Err(error) => {
            set_disconnected_state(connection_state.inner());
            return Err(error);
        }
    };

    let (shutdown_sender, shutdown_receiver) = oneshot::channel();
    let task_handle = runtime_guard.spawn(run_connection_task(
        stream,
        shutdown_receiver,
        Arc::clone(connection_state.inner()),
    ));

    let mut finalize_guard = match connection_state.lock() {
        Ok(guard) => guard,
        Err(_) => {
            let _ = shutdown_sender.send(());
            let _ = runtime_guard.block_on(async {
                timeout(Duration::from_secs(DISCONNECT_TIMEOUT_SECONDS), task_handle).await
            });
            return Err(anyhow!("failed to lock connection state for finalization"));
        }
    };

    finalize_guard.lifecycle_state = ConnectionLifecycleState::Connected(ConnectionHandle {
        shutdown_sender: Some(shutdown_sender),
        task_handle,
        connected_host: host.clone(),
        connected_port: port,
        connected_at: Instant::now(),
    });

    Ok(ConnectionCommandResult {
        is_connected: true,
        message: format!("Connected to {}", address),
    })
}
