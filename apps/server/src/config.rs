#![allow(dead_code)]

use anyhow::{Result, anyhow};
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DatabaseSettings {
    addr: String,
    port: u32,
    user: String,
    pass: String,
    name: String,
    kind: String,
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".into(),
            port: 5432,
            user: "undertone_user".into(),
            pass: "undertone_pass".into(),
            name: "undertone_datbase".into(),
            kind: "pgsql".into(),
        }
    }
}

/// Represents the server settings as parsed from configuration files.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    server_name: String,
    server_desc: String,
    server_port: u32,
    server_icon: Option<String>,
    admin_only: bool,
    db: DatabaseSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_name: "My Undertone Server".into(),
            server_desc: "A **NEW** Undertone Community Server".into(),
            server_port: 9990,
            server_icon: None,
            admin_only: false,
            db: DatabaseSettings {
                ..Default::default()
            },
        }
    }
}

impl Settings {
    /// Instantiate a new Settings loaded from the following files:
    /// undertone.<extension>
    /// local.<extension>
    /// dev.<extension>
    ///
    /// Settings are layered in the above order so dev will overwrite local and local will
    /// overwrite undertone. This allows for testing and special dev settings based on
    /// environment.
    pub fn new() -> Result<Self> {
        let settings: Settings = Config::builder()
            .add_source(File::with_name("config/undertone").required(true))
            .add_source(File::with_name("config/local").required(false))
            .add_source(File::with_name("config/dev").required(false))
            .build()?
            .try_deserialize()?;

        Ok(settings)
    }

    /// Loads config from custom paths. The path should not include the file extension
    /// as format is auto detected. Paths should not use extensions as they will be
    /// automatically detected eg. `config/undertone` will load undertone.toml, undertone.json etc.
    ///
    /// # Errors
    /// If no paths are provided.
    /// If invalid config is detected.
    pub fn from_paths(paths: Vec<String>) -> Result<Self> {
        if paths.is_empty() {
            return Err(anyhow!("No paths provided!"));
        }

        let mut builder = Config::builder();
        for path in paths {
            builder = builder.add_source(File::with_name(&path).required(true));
        }

        let settings: Settings = builder.build()?.try_deserialize()?;
        Ok(settings)
    }

    /// Loads config using string literal representations of the config file. You must specify
    /// the format for the strings using config::FileFormat.
    ///
    /// # Errors
    /// If no strings are provided.
    /// If invalid strings are provided.
    fn from_string_literals(configs: Vec<String>, format: FileFormat) -> Result<Self> {
        if configs.is_empty() {
            return Err(anyhow!("No config strings received!"));
        }

        let mut builder = Config::builder();
        for config in configs {
            builder = builder.add_source(File::from_str(&config, format));
        }

        let settings: Settings = builder.build()?.try_deserialize()?;
        Self::validate_settings(&settings)?;
        Ok(settings)
    }

    /// Validate settings based on individual requirements. Not all settings are validated.
    fn validate_settings(settings: &Self) -> Result<()> {
        let mut bad: Vec<&str> = vec![];

        // Test the important settings.
        if settings.db.addr.is_empty() {
            bad.push("db.addr is empty...");
        }
        if settings.db.port < 1024 {
            bad.push("db_port is in restricted range, must be greater than 1023...");
        }
        if settings.db.port >= u16::MAX.into() {
            bad.push("db_port is higher than maximum possible port, must be less than 65,535");
        }
        if settings.db.user.is_empty() {
            bad.push("db_user is empty...");
        }
        if settings.db.pass.is_empty() {
            bad.push("db_pass is empty...");
        }
        if settings.db.name.is_empty() {
            bad.push("db_name is empty...");
        }

        if !bad.is_empty() {
            let mut error_string: Vec<String> = vec![];
            for field in bad {
                error_string.push(field.to_string());
            }

            return Err(anyhow!(error_string.join("")));
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn to_yaml(val: &Settings) -> String {
        serde_saphyr::to_string::<Settings>(val).expect("failed to serialize yaml")
    }
    fn to_ron(val: &Settings) -> String {
        ron::to_string::<Settings>(val).expect("failed to serialize to ron")
    }
    fn to_json(val: &Settings) -> String {
        serde_json::to_string_pretty::<Settings>(val).expect("failed to serialize json")
    }
    fn to_toml(val: &Settings) -> String {
        toml::to_string_pretty::<Settings>(val).expect("failed to serialize to TOML")
    }

    #[test]
    fn loads_config_from_strings() {
        let default_settings = Settings::default();

        let toml = to_toml(&default_settings);

        let settings = Settings::from_string_literals(vec![toml], FileFormat::Toml).unwrap();

        // Assert different variable types to ensure they are surviving the round trip.
        assert_eq!(settings.server_name, "My Undertone Server".to_string());
        assert_eq!(settings.server_port, 9990);
        assert!(!settings.admin_only);
    }

    #[test]
    fn parses_json() {
        let default_settings = Settings::default();
        let json = to_json(&default_settings);
        let settings = Settings::from_string_literals(vec![json], FileFormat::Json).unwrap();

        // Assert different variable types to ensure they are surviving the round trip.
        assert_eq!(settings.server_name, "My Undertone Server".to_string());
        assert_eq!(settings.server_port, 9990);
        assert!(!settings.admin_only);
    }

    #[test]
    fn parses_ron() {
        let default_settings = Settings::default();
        let ron = to_ron(&default_settings);
        let settings = Settings::from_string_literals(vec![ron], FileFormat::Ron).unwrap();

        // Assert different variable types to ensure they are surviving the round trip.
        assert_eq!(settings.server_name, "My Undertone Server".to_string());
        assert_eq!(settings.server_port, 9990);
        assert!(!settings.admin_only);
    }

    #[test]
    fn parses_yaml() {
        let default_settings = Settings::default();
        let yaml = to_yaml(&default_settings);
        let settings = Settings::from_string_literals(vec![yaml], FileFormat::Yaml).unwrap();

        // Assert different variable types to ensure they are surviving the round trip.
        assert_eq!(settings.server_name, "My Undertone Server".to_string());
        assert_eq!(settings.server_port, 9990);
        assert!(!settings.admin_only);
    }

    #[test]
    fn fails_on_empty_settings() {
        let mut default_settings = Settings::default();
        default_settings.db.addr = "".into();

        let toml = to_toml(&default_settings);

        assert!(
            Settings::from_string_literals(vec![toml], FileFormat::Toml)
                .unwrap_err()
                .to_string()
                .contains("is empty")
        )
    }

    #[test]
    fn fails_on_reserved_port() {
        let mut default_settings = Settings::default();
        default_settings.db.port = 69;

        let toml = to_toml(&default_settings);

        assert!(
            Settings::from_string_literals(vec![toml], FileFormat::Toml)
                .unwrap_err()
                .to_string()
                .contains("restricted")
        )
    }

    #[test]
    fn fails_on_out_of_range_port() {
        let mut default_settings = Settings::default();
        default_settings.db.port = 70000;

        let toml = to_toml(&default_settings);

        assert!(
            Settings::from_string_literals(vec![toml], FileFormat::Toml)
                .unwrap_err()
                .to_string()
                .contains("maximum possible port")
        )
    }

    #[test]
    fn load_from_files() {
        let settings = Settings::new().expect("failed to creating settings from files.");

        assert_eq!(settings.server_name, "A Default Undertone Server");
        assert_eq!(settings.server_port, 9991);
        assert!(settings.admin_only);
        assert_eq!(settings.db.kind, "mysql");
    }
}
