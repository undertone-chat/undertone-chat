use std::env;

use anyhow::{Result, anyhow};
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct Settings {
    /// Stored in env as UNDERTONE_DEBUG=
    //pub debug: bool,
    pub general: General,
    pub security: Option<Security>,
    pub database: Option<Database>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct General {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Security {
    pub admin_only: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub uri: String,
}

impl Settings {
    pub fn new() -> Result<Self> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Load standard  undertone config.
            .add_source(File::with_name("undertone"))
            // Search for any specific configurations for run mode (eg. development or production)
            .add_source(File::with_name(&format!("{run_mode}")).required(false))
            // Load local file (Do not commit this to the repo, holds sensitive data)
            .add_source(File::with_name("local").required(false))
            // Load any and all environment variables prefixed with UNDERTONE
            .add_source(Environment::with_prefix("UNDERTONE"))
            .build()?;

        // Attempt to freeze into our Settings struct.
        match s.try_deserialize::<Settings>() {
            Ok(result) => Ok(result),
            Err(error) => Err(anyhow!(
                "Failed to deserialize settings to struct: {:?}",
                error
            )),
        }
    }
}
