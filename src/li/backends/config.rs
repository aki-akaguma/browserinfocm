use anyhow::{bail, Context, Result};
use config::{Config, Environment, File, FileFormat};
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize, Debug, Clone)]
pub struct BackendConfig {
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseConfig {
    pub base_path: String,
    pub db_file: String,
}

static CONFIG: OnceLock<BackendConfig> = OnceLock::new();

impl BackendConfig {
    pub fn init() -> Result<()> {
        let config = Self::load().context("Failed to load config")?;
        if let Err(_v) = CONFIG.set(config) {
            bail!("Failed to set global config");
        }
        Ok(())
    }

    fn load() -> anyhow::Result<Self> {
        let default_toml = r#"
[database]
base_path = "/var/local/data/browserinfocm"
db_file = "browserinfocm.sqlite3"
"#;

        let s = Config::builder()
            // 1. Load defaults
            .add_source(File::from_str(default_toml, FileFormat::Toml))
            // 2. Load from config.toml if it exists
            .add_source(File::with_name("browserinfocm").required(false))
            // 3. Environment variables (e.g., CATTONGUE__DATABASE__BASE_PATH)
            .add_source(Environment::with_prefix("BROWSERINFOCM").separator("__"))
            .build()?;

        let config: BackendConfig = s.try_deserialize()?;
        Ok(config)
    }

    pub fn global() -> &'static BackendConfig {
        CONFIG.get().expect("Config is not initialized")
    }
}
