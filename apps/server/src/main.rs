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
    let (settings, used_default_settings) = match Settings::new() {
        Ok(settings) => (settings, false),
        Err(error) => {
            tracing::warn!(error = %error, "Failed to load configuration; starting with default settings");
            (Settings::default(), true)
        }
    };

    // Print some basic config settings for log on start.
    if used_default_settings {
        tracing::info!("Default config loaded for: {}", settings.server_name);
    } else {
        tracing::info!("Config loaded for {}", settings.server_name);
    }

    Ok(())
}
