mod config;

use config::Settings;
use std::str::from_utf8;

use anyhow::{Result, anyhow};
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Builder;

fn main() -> Result<()> {
    // Setup tracing subscriber.
    tracing_subscriber::fmt::init();

    // Display some basic information about the running process, useful for debugging with users.
    tracing::info!(
        "Starting {} v{} on {}:{} Arch: {}",
        option_env!("CARGO_PKG_NAME").unwrap_or("Unknown Package"),
        option_env!("CARGO_PKG_VERSION").unwrap_or("X.X.X"),
        std::env::consts::FAMILY,
        std::env::consts::OS,
        std::env::consts::ARCH,
    );

    let settings = Settings::new()?;

    tracing::debug!(
        "Did we get anything? Name: {}, Description: {}",
        settings.general.name,
        settings.general.description
    );

    Ok(())
}
