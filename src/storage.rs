use crate::Recipe;
use crate::models::Database;

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const FILENAME: &str = "/tmp/dina_data.toml";

pub struct Storage;

impl Storage {
    // -- Check where the project is located/installed
    pub fn get_db_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "dina", "dina-cli")
            .context("Could not determine project directory")?;

        // -- Create directory
        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir)?;

        // -- Create filename
        Ok(data_dir.join(FILENAME))
    }

    pub fn load() -> Result<Database> {
        let path = Self::get_db_path()?;

        if !path.exists() {
            return Ok(Database::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read database at {:?}", path))?;

        toml::from_str(&content).with_context(|| format!("Failed to parse database at {:?}", path))
    }

    pub fn save(db: &Database) -> Result<()> {
        let path = Self::get_db_path()?;
        let content = toml::to_string_pretty(db).context("Failed to serialize database")?;

        fs::write(&path, content).with_context(|| format!("Failed to write database to {:?}", path))
    }

    pub fn add_recipe(recipe: Recipe) -> Result<()> {
        let mut db = Self::load()?;

        // Überprüfe auf Duplikate
        if db.recipes.iter().any(|r| r.name == recipe.name) {
            anyhow::bail!("Recipe '{}' already exists", recipe.name);
        }

        db.recipes.push(recipe);
        Self::save(&db)
    }
}
