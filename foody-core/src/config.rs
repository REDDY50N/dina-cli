use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DatabaseType {
    Toml,
    Sqlite,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub db_type: DatabaseType,
    pub connection_string: String,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DinaConfig {
    pub database: DatabaseConfig,
    pub recipe_dir: PathBuf,
    pub data_dir: PathBuf,
    pub git_remote: Option<String>,
}

impl Default for DinaConfig {
    fn default() -> Self {
        let proj_dirs = ProjectDirs::from("com", "foody", "foody-cli")
            .expect("Could not determine project directory");

        let data_dir = proj_dirs.data_dir().to_path_buf();
        let recipe_dir = data_dir.join("recipes");

        DinaConfig {
            database: DatabaseConfig {
                db_type: DatabaseType::Toml,
                connection_string: data_dir.join("foody.sqlite").to_str().unwrap().to_string(),
            },
            recipe_dir,
            data_dir,
            git_remote: None,
        }
    }
}

// ... rest bleibt gleich