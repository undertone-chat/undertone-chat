use std::env;

use anyhow::{Result, anyhow};
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub(crate) struct Settings {
    /// Stored in env as UNDERTONE_DEBUG=
    //pub debug: bool,
    pub general: General,
    pub security: Security,
    pub database: Database,
}

impl Settings {
    /// Produce a valid uri for PostgreSQL
    pub fn get_db_uri(&self) -> String {
        format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.database.username,
            self.database.password,
            self.database.address,
            self.database.port,
            self.database.database
        )
    }
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct General {
    pub name: String,
    pub description: String,
    pub port: usize,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Security {
    pub admin_only: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub address: String,
    pub port: String,
    pub database: String,
    pub username: String,
    pub password: String,
}

impl Settings {
    pub fn new() -> Result<Self> {
        tracing::debug!("Getting run mode.");
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        tracing::debug!("Got run mode {}.  Loading files.", run_mode);

        let s = Config::builder()
            // Default Values for non Optional fields.
            .set_default("general.name", "My Undertone Server")?
            .set_default("general.description", "A new Undertone community")?
            .set_default("general.port", "9990")?
            .set_default("database.address", "127.0.0.1")?
            .set_default("database.port", "5432")?
            .set_default("database.username", "defaultUser")?
            .set_default("database.database", "defaultDatabase")?
            .set_default("database.password", "defaultPassword")?
            .set_default("security.admin_only", false)?
            // Load standard  undertone config.
            .add_source(File::with_name("undertone").required(false))
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loads_config_file() {
        let settings = Settings::new().unwrap();
        assert_eq!(settings.general.name, "My Undertone Server".to_string());
        assert_eq!(
            settings.general.description,
            "A new Undertone community".to_string()
        );
    }

    #[test]
    fn produce_valid_db_uri() {
        let expected_uri =
            "postgresql://defaultUser:defaultPassword@127.0.0.1:5432/defaultDatabase".to_string();
        let settings = Settings::new().unwrap();

        assert_eq!(settings.get_db_uri(), expected_uri);
    }
}
