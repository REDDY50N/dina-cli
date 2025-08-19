//use crate::Article;
use crate::Recipe;
// use crate::models::Database;
use crate::models::{Article, Database, MealPlan};

use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

// -- Database filename (TOML)
const FILENAME: &str = "/home/polar/.local/share/dina-cli/dina_data.toml";

// -- Storage / Database struct
pub struct Storage;

// ----------------------------------------------
// INIT DATABASE
// ----------------------------------------------
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
}

// ----------------------------------------------
// GET DB DIR
// ----------------------------------------------
impl Storage {
    pub fn get_db_dir() -> anyhow::Result<std::path::PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "dina", "dina-cli")
            .context("Could not determine project directory")?;

        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;

        Ok(data_dir.to_path_buf())
    }
}

// ----------------------------------------------
// LOAD DATABASE
// ----------------------------------------------
impl Storage {
    pub fn load() -> Result<Database> {
        let path = Self::get_db_path()?;

        if !path.exists() {
            return Ok(Database::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read database at {:?}", path))?;

        toml::from_str(&content).with_context(|| format!("Failed to parse database at {:?}", path))
    }
}

// ----------------------------------------------
// WRITE TO DATABASE
// ----------------------------------------------
impl Storage {
    pub fn save(db: &Database) -> Result<()> {
        let path = Self::get_db_path()?;
        let content = toml::to_string_pretty(db).context("Failed to serialize database")?;
        fs::write(&path, content).with_context(|| format!("Failed to write database to {:?}", path))
    }
}

// ----------------------------------------------
// LIST ARTICLES
// ----------------------------------------------
impl Storage {
    pub fn list_articles() -> anyhow::Result<Vec<Article>> {
        let db = Self::load()?;
        Ok(db.articles)
    }
}

// ----------------------------------------------
// ADD RECIPE
// ----------------------------------------------
impl Storage {
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

// ----------------------------------------------
// ADD ARTICLE
// ----------------------------------------------
impl Storage {
    pub fn add_article(article: Article) -> Result<()> {
        let mut db = Self::load()?;

        // Prüfe, ob der Artikel bereits existiert
        if let Some(index) = db.articles.iter().position(|a| a.name == article.name) {
            // Artikel existiert - Menge aktualisieren
            db.articles[index].quantity += article.quantity;
        } else {
            // Neuer Artikel - hinzufügen
            db.articles.push(article);
        }

        Self::save(&db)
    }
}

// ----------------------------------------------
// ADD MEAL (MEAL PLAN)
// ----------------------------------------------
impl Storage {
    pub fn add_meal(meal: MealPlan) -> Result<()> {
        let mut db = Self::load()?;

        // Entferne vorhandene Mahlzeit für den gleichen Tag
        db.meal_plans
            .retain(|m| m.day.to_lowercase() != meal.day.to_lowercase());

        db.meal_plans.push(meal);
        Self::save(&db)
    }

    pub fn list_meals() -> Result<Vec<MealPlan>> {
        let db = Self::load()?;
        Ok(db.meal_plans)
    }
}

// ----------------------------------------------
// DELETE RECIPE
// ----------------------------------------------
impl Storage {
    pub fn delete_recipe(name: &str) -> Result<()> {
        let mut db = Self::load()?;
        let original_len = db.recipes.len();
        db.recipes
            .retain(|r| r.name.to_lowercase() != name.to_lowercase());

        if db.recipes.len() == original_len {
            anyhow::bail!("Rezept '{}' nicht gefunden", name);
        }

        Self::save(&db)
    }
}

// ----------------------------------------------
// DELETE MEAL (MEAL PLAN)
// ----------------------------------------------
impl Storage {
    pub fn delete_meal(day: &str) -> Result<()> {
        let mut db = Self::load()?;
        let original_len = db.meal_plans.len();
        db.meal_plans
            .retain(|m| m.day.to_lowercase() != day.to_lowercase());

        if db.meal_plans.len() == original_len {
            anyhow::bail!("Mahlzeit für '{}' nicht gefunden", day);
        }

        Self::save(&db)
    }
}

// ----------------------------------------------
// DELETE ARTICLE
// ----------------------------------------------
impl Storage {
    pub fn delete_article(name: &str) -> Result<()> {
        let mut db = Self::load()?;
        let original_len = db.articles.len();
        db.articles
            .retain(|a| a.name.to_lowercase() != name.to_lowercase());

        if db.articles.len() == original_len {
            anyhow::bail!("Artikel '{}' nicht gefunden", name);
        }

        Self::save(&db)
    }
}

// ----------------------------------------------
// DELETE SHOPPING LIST ITEM
// ----------------------------------------------
impl Storage {
    // Shopping-Item löschen nach ID
    pub fn delete_shopping_item(id: usize) -> Result<()> {
        let mut db = Self::load()?;
        let original_len = db.shopping_list.len();
        db.shopping_list.retain(|i| i.id != id);

        if db.shopping_list.len() == original_len {
            anyhow::bail!("Einkaufslisten-Item mit ID {} nicht gefunden", id);
        }

        Self::save(&db)
    }
}
