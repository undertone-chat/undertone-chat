use anyhow::{Context, Result, anyhow};
use config::{Config, File, FileFormat, Source, Value};
use serde::Deserialize;
macro_rules! get_required_config {
    ($obj:expr, $name:literal) => {
        $obj.get_string($name)
            .with_context(|| format!("failed to parse required '{}'", $name))
    };
}

#[derive(Debug, Clone, Copy)]
pub struct SettingMeta {
    pub field: &'static str,
    pub key: &'static str,
    pub ty: &'static str,
    pub default: &'static str,
    pub private: bool,
    pub required: bool,
}

macro_rules! define_settings {
    (
        $(
            $field:ident : $ty:ty {
                key: $key:literal,
                default: $default:expr,
                private: $private:expr,
                required: $required:expr,
            }
        ),* $(,)?
    ) => {
        #[derive(Debug, Clone, Deserialize)]
        pub struct Settings {
            $(pub $field: $ty,)*
        }

        impl Default for Settings {
            fn default() -> Self {
                Self {
                    $($field: $default,)*
                }
            }
        }

        impl Settings {
            pub fn metadata() -> &'static [SettingMeta] {
                &[
                    $(
                        SettingMeta {
                            field: stringify!($field),
                            key: $key,
                            ty: stringify!($ty),
                            default: stringify!($default),
                            private: $private,
                            required: $required,
                        }
                    ),*
                ]
            }

            pub fn parse_file_strings(base: &str, prv: &str, format: FileFormat) -> Result<Self> {
                let mut builder = config::Config::builder();

                $(builder = builder.set_default($key, $default)?;)*

                let base_config = Config::builder()
                    .add_source(File::from_str(base, format))
                    .build()?;

                for meta in Settings::metadata() {
                    if meta.private {
                        if let Ok(_) = base_config.get::<config::Value>(meta.key) {
                            return Err(anyhow!(
                                    "private settings `{}` was defined in your base config; move it to `.undertone_private`",
                                    meta.key
                                    ))
                        }
                    }
                }

                let config = builder
                .add_source(base_config)
                .add_source(File::from_str(prv, format))
                .build()?;

                let settings: Settings = config.try_deserialize()?;
                settings.validate()?;

                Ok(settings)
            }

            pub fn validate(&self) -> Result<()> {
                $(
                    if $required {
                        validate_required(stringify!($field), &self.$field)?;
                    }
                    )*
                    Ok(())
            }
        }
    }
}

trait RequiredCheck {
    fn is_missing(&self) -> bool;
}

impl RequiredCheck for String {
    fn is_missing(&self) -> bool {
        self.trim().is_empty()
    }
}

impl RequiredCheck for bool {
    fn is_missing(&self) -> bool {
        false
    }
}

impl RequiredCheck for u32 {
    fn is_missing(&self) -> bool {
        false
    }
}

impl RequiredCheck for i32 {
    fn is_missing(&self) -> bool {
        false
    }
}

impl RequiredCheck for f32 {
    fn is_missing(&self) -> bool {
        false
    }
}

fn validate_required<T: RequiredCheck>(field: &'static str, value: &T) -> Result<()> {
    if value.is_missing() {
        Err(anyhow!("required setting `{}` is missing", field))
    } else {
        Ok(())
    }
}

define_settings! {
    server_name: String {
        key: "server_name",
        default: "My Undertone Server".to_string(),
        private: false,
        required: true,
    },
    server_description: String {
        key: "server_desc",
        default: "A new Undertone community server.".to_string(),
        private: false,
        required: true,
    },
    icon_path: String {
        key: "icon_path",
        default: "".to_string(),
        private: false,
        required: false,
    },
    admin_only: bool {
        key: "admin_only",
        default: false,
        private: false,
        required: true,
    },
    db_kind: String {
        key: "db_kind",
        default: "pgsql".to_string(),
        private: true,
        required: true,
    },
    db_addr: String {
        key: "db_addr",
        default: "".to_string(),
        private: true,
        required: true,
    },
    db_port: u32 {
        key: "db_port",
        default: 5432,
        private: true,
        required: true,
    },
    db_user: String {
        key: "db_user",
        default: "".to_string(),
        private: true,
        required: true,
    },
    db_pass: String {
        key: "db_pass",
        default: "".to_string(),
        private: true,
        required: true,
    },
    db_name: String {
        key: "db_name",
        default: "".to_string(),
        private: true,
        required: true,
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
    fn accepts_yaml_config() {
        let base = r#"
        server_name: YAML Test Server
        server_description: A Test Server
        admin_only: true
        icon_path: a/path/to/the/icon.png
        "#;

        let private = r#"
                db_kind: pgsql
                db_addr: 127.0.0.1
                db_port: 2345
                db_user: YamlUser
                db_pass: YamlPassword
                db_name: YamlDatabase
            "#;

        assert_eq!(
            Settings::parse_file_strings(base, private, FileFormat::Yaml)
                .unwrap()
                .server_name,
            "YAML Test Server".to_string()
        )
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
