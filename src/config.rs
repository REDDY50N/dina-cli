use crate::database::{DatabaseConfig, DatabaseType};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DinaConfig {
    pub database: DatabaseConfig,
    pub recipe_dir: PathBuf,
    pub data_dir: PathBuf,
    pub git_remote: Option<String>,
}

impl Default for DinaConfig {
    fn default() -> Self {
        let proj_dirs = ProjectDirs::from("com", "dina", "dina-cli")
            .expect("Could not determine project directory");

        let data_dir = proj_dirs.data_dir().to_path_buf();
        let recipe_dir = data_dir.join("recipes");

        DinaConfig {
            database: DatabaseConfig {
                db_type: DatabaseType::Sqlite,
                connection_string: data_dir.join("dina.sqlite").to_str().unwrap().to_string(),
            },
            recipe_dir,
            data_dir,
            git_remote: None,
        }
    }
}

impl DinaConfig {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: DinaConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = DinaConfig::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> anyhow::Result<()> {
        let config_path = Self::get_config_path()?;
        std::fs::create_dir_all(config_path.parent().unwrap())?;

        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;

        Ok(())
    }

    fn get_config_path() -> anyhow::Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "dina", "dina-cli")
            .ok_or_else(|| anyhow::anyhow!("Could not determine project directory"))?;

        let config_dir = proj_dirs.config_dir();
        Ok(config_dir.join("config.toml"))
    }

    pub fn ensure_directories(&self) -> anyhow::Result<()> {
        std::fs::create_dir_all(&self.data_dir)?;
        std::fs::create_dir_all(&self.recipe_dir)?;
        Ok(())
    }
}
