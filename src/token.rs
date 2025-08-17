// Github Token-Verwltung

use anyhow::Context;
use anyhow::Result;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const TOKEN_FILE: &str = "github_token.txt";

pub struct TokenManager;

impl TokenManagr {
    pub fn get_token_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "dina", "dina-cli")
            .context("Could not determine project directory")?;

        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)?;

        Ok(config_dir.join(TOKEN_FILE))
    }

    pub fn save_token(token: &str) -> Result<()> {
        let path = Self::get_token_path()?;
        fs::write(path, token)?;
        Ok(())
    }

    pub fn load_token() -> Result<Option<String>> {
        let path = Self::get_token_path()?;

        if path.exists() {
            let token = fs::read_to_string(path)?;
            Ok(Some(token.trim().to_string()))
        } else {
            Ok(None)
        }
    }
}
