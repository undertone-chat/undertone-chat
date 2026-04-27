use std::env;

use anyhow::{Context, Result, anyhow};
use config::{Config, File, FileFormat};
macro_rules! get_required_config {
    ($obj:expr, $name:literal) => {
        $obj.get_string($name)
            .with_context(|| format!("failed to parse required '{}'", $name))
    };
}

/// Stores server settings loaded from configuration files.
#[doc(alias = "setting")]
#[allow(unused)]
#[derive(Debug)]
pub struct Settings {
    pub server_name: String,
    pub server_description: String,
    pub icon_path: Option<String>,
    pub admin_only_access: bool,
    pub db_kind: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_addr: String,
    pub db_port: String,
    pub db_name: String,
}

impl Settings {
    /// Parses strings from files into config object and validates and verifies configuration.
    /// accepts strings in common configuration formats:
    /// TOML, JSON, YAML, INI, RON, JSON5, CORN
    ///
    /// # Panics
    ///
    /// Panics if required settings are omitted from configurations.
    /// Panics if format is incorrect in provided &str
    pub fn new(base: &str, private: &str, format: FileFormat) -> Result<Self> {
        let private_only: Vec<&str> = vec![
            "db_user", "db_pass", "db_addr", "db_port", "db_name", "db_kind",
        ];

        tracing::debug!("Getting run mode.");

        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        tracing::debug!("Got run mode {}.  Loading files.", run_mode);

        let base_config = Config::builder()
            .add_source(File::from_str(base, format))
            .build()?;

        // Verify we do not have any private settings in our base config file.
        for key in private_only {
            if base_config.get_string(key).is_ok() {
                return Err(anyhow!(
                    "Private setting '{}' found in base config. This should only be set in the '.undertone_private' configuration.",
                    key
                ));
            }
        }

        let private_config = Config::builder()
            .add_source(File::from_str(private, format))
            .build()?;

        // Build our settings struct with default values for missing values, err on required values.
        let settings = Settings {
            server_name: base_config
                .get("server_name")
                .unwrap_or("My Undertone Server".to_string()),
            server_description: base_config
                .get("server_description")
                .unwrap_or("A new Undertone comunnity server".to_string()),
            admin_only_access: base_config.get("admin_only_access").unwrap_or(false),
            icon_path: base_config.get("icon_path").ok(),
            db_kind: private_config.get("db_kind").unwrap_or("pgsql".to_string()),
            db_addr: get_required_config!(private_config, "db_addr")?,
            db_port: get_required_config!(private_config, "db_port")?,
            db_user: get_required_config!(private_config, "db_user")?,
            db_pass: get_required_config!(private_config, "db_pass")?,
            db_name: get_required_config!(private_config, "db_name")?,
        };

        Ok(settings)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Helpers
    fn default_base() -> String {
        r#"
        server_name='Toml Test Server'
        server_description='A Test Server'
        admin_only=true
        icon_path='a/path/to/the/icon.png'
        "#
        .to_string()
    }

    fn default_private() -> String {
        r#"
        db_kind='pgsql'
        db_addr='127.0.0.2'
        db_port='1234'
        db_user='TestUser'
        db_pass='TestPass'
        db_name='TestDatabase'
        "#
        .to_string()
    }

    #[test]
    fn accepts_json_config() {
        let base = r#"
        {
            "server_name": "JSON Test Server",
            "server_description": "A Test Server",
            "admin_only": true,
            "icon_path": "a/path/to/the/icon.png"
        }"#;

        let private = r#"
            {
                "db_kind": "pgsql",
                "db_addr": "127.0.0.1",
                "db_port": "2345",
                "db_user": "JsonUser",
                "db_pass": "JsonPassword",
                "db_name": "JsonDatabase"
            }
            "#;

        assert_eq!(
            Settings::new(base, private, FileFormat::Json)
                .unwrap()
                .server_name,
            "JSON Test Server".to_string()
        )
    }

    #[test]
    fn accepts_toml_config() {
        let base = default_base();
        let private = default_private();
        let kind = FileFormat::Toml;

        let settings = Settings::new(&base, &private, kind).unwrap();

        assert_eq!(settings.server_name, "Toml Test Server".to_string());
    }

    #[test]
    fn rejects_private_settings_in_base() {
        let base = r#"
            server_name='Test Server'
            server_description='A Test Server'
            admin_only=true
            icon_path='a/path/to/the/icon.png'
            db_addr='127.0.0.2'
            "#;

        let private = default_private();
        let result = Settings::new(base, &private, FileFormat::Toml).unwrap_err();
        assert!(result.to_string().contains("Private setting"));
    }

    #[test]
    fn rejects_missing_required_settings() {
        let base = default_base();
        let private = r#"
            db_kind='pgsql'
            db_port='1234'
            db_user='TestUser'
            db_pass='TestPass'
            db_name='TestDatabase'
            "#;

        let result = Settings::new(&base, private, FileFormat::Toml).unwrap_err();
        assert!(
            result
                .to_string()
                .contains("failed to parse required 'db_addr'")
        )
    }

    #[test]
    fn fail_creation_if_missing_database_uri() {}
}
