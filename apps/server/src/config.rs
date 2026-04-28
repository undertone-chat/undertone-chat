#![allow(dead_code)]

use anyhow::{Context, Result, anyhow};
use config::{Config, File, FileFormat, Source, Value};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

#[serde(default)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    server_name: String,
    server_desc: String,
    server_port: u32,
    server_icon: Option<String>,
    db: DatabaseSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_name: "My Undertone Server".into(),
            server_desc: "A **NEW** Undertone Community Server".into(),
            server_port: 9990,
            server_icon: None,
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
            .add_source(File::from_name("undertone").required(true))
            .add_source(File::from_name("local").required(false))
            .add_source(File::from_name("dev").required(false))
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
    pub fn from_paths(paths: Vec<&str>) -> Result<Self> {
        if paths.is_empty() {
            return Err(anyhow!("No paths provided!"));
        }

        let mut builder = Config::builder();
        for path in paths {
            builder = builder.add_source(File::with_name(path).required(true));
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
    fn from_string_literals(configs: Vec<&str>, format: FileFormat) -> Result<Self> {
        if configs.is_empty() {
            return Err(anyhow!("No config strings received!"));
        }

        let mut builder = Config::builder();
        for config in configs {
            builder = builder.add_source(File::from_str(config, format));
        }

        let settings: Settings = builder.build()?.try_deserialize()?;
        Ok(settings)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    server_name: String,
    server_desc: String,
    server_port: u32,
    server_icon: Option<String>,
    #[serde(default)]
    db: DatabaseSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_name: "My Undertone Server".into(),
            server_desc: "A **NEW** Undertone Community Server".into(),
            server_port: 9990,
            server_icon: None,
            db: DatabaseSettings {
                ..Default::default()
            },
        }
    }
}
    }
}
#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::*;
    use serde_json::{Map, Number, Value};

    fn metadata_to_json() -> (Map<String, Value>, Map<String, Value>) {
        let mut public = Map::new();
        let mut private = Map::new();

        for m in Settings::metadata() {
            let v = match m.ty {
                "String" => Value::String(m.default.to_string()),
                "bool" => Value::Bool(m.default.parse().unwrap()),
                "u32" => Value::Number(Number::from_str(m.default).unwrap()),
                "i32" => Value::Number(Number::from_str(m.default).unwrap()),
                "f32" => Value::Number(Number::from_str(m.default).unwrap()),
                _ => panic!("unsupported test type {}", m.ty),
            };

            if m.private {
                private.insert(m.key.to_string(), v);
            } else {
                public.insert(m.key.to_string(), v);
            }
        }

        (public, private)
    }

    fn json_fixtures() -> (String, String) {
        let (public, private) = metadata_to_json();

        (
            serde_json::to_string_pretty(&public).unwrap(),
            serde_json::to_string_pretty(&private).unwrap(),
        )
    }

    fn yaml_fixtures() -> (String, String) {
        let (public, private) = metadata_to_json();
        (
            serde_saphyr::to_string(&public).unwrap(),
            serde_saphyr::to_string(&private).unwrap(),
        )
    }

    fn toml_fixtures() -> (String, String) {
        let (public, private) = metadata_to_json();

        let toml_public = toml::Value::try_from(public).unwrap();
        let toml_private = toml::Value::try_from(private).unwrap();
        (
            toml::to_string_pretty(&toml_public).unwrap(),
            toml::to_string_pretty(&toml_private).unwrap(),
        )
    }

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
    fn loads_yaml() {
        let (public, private) = yaml_fixtures();
        let settings = Settings::parse_file_strings(&public, &private, FileFormat::Yaml).unwrap();

        assert_eq!(settings.server_name, "My Undertone Server");
        assert!(!settings.admin_only);
        assert_eq!(settings.db_port, 5432);
    }

    #[test]
    fn accepts_ini_config() {
        let base = r#"
        server_name=INI Test Server
        server_description=A Test Server
        admin_only=true
        icon_path=a/path/to/the/icon.png
        "#;

        let private = r#"
            db_kind=pgsql
            db_addr=127.0.0.1
            db_port=2345
            db_user=IniUser
            db_pass=IniPassword
            db_name=IniDatabase
            "#;

        assert_eq!(
            Settings::parse_file_strings(base, private, FileFormat::Ini)
                .unwrap()
                .server_name,
            "INI Test Server".to_string()
        )
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
            Settings::parse_file_strings(base, private, FileFormat::Json)
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

        let settings = Settings::parse_file_strings(&base, &private, kind).unwrap();

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
        let result = Settings::parse_file_strings(base, &private, FileFormat::Toml).unwrap_err();
        assert!(result.to_string().contains("private settings"));
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

        let result = Settings::parse_file_strings(&base, private, FileFormat::Toml).unwrap_err();
        let error_string = result.to_string();
        assert!(error_string.contains("Missing") || error_string.contains("missing"));

        let private = r#"
            db_addr=''
            db_kind='pgsql'
            db_port=1234
            db_user='TestUser'
            db_pass='TestPass'
            db_name='TestDatabase'
            "#;
        let result = Settings::parse_file_strings(&base, private, FileFormat::Toml).unwrap_err();
        let error_string = result.to_string();
        assert!(error_string.contains("Missing") || error_string.contains("missing"));
    }

    #[test]
    fn fail_creation_if_missing_database_uri() {}
}
