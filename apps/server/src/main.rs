mod config;

use anyhow::Result;
use config::Settings;

fn main() -> Result<()> {
    // Setting up tracing subscriber.
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

    // TODO: Add support for command line pointers to user specified config files.
    let settings = Settings::new()?;

    // Print some basic config settings for log on start.
    tracing::info!("Config loaded for {}", settings.server_name);
    Ok(())
}
