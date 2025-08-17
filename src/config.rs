//pub const FILENAME: &str = "/tmp/dina_data.toml";
//pub const FILENAME: &str = "/home/polar/CODE/database/dina-db.toml";

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub auto_sync: bool,
}

const CONFIG_FILE: &str = "config.toml";

pub struct ConfigManager;

impl ConfigManager {
    pub fn get_config_path() -> Result<PathBuf> {
        // Ähnlich wie TokenManager
    }

    pub fn load() -> Result<Config> {
        let path = Self::get_config_path()?;
        if path.exists() {
            let content = fs::read_to_string(path)?;
            toml::from_str(&content).context("Failed to parse config")
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(config: &Config) -> Result<()> {
        let path = Self::get_config_path()?;
        let content = toml::to_string_pretty(config)?;
        fs::write(path, content)?;
        Ok(())
    }
}
