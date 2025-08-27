use crate::models::{Article, Database, MealPlan, ShoppingItem};

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
// LIST SHOPPING LIST
// ----------------------------------------------
impl Storage {
    pub fn list_shopping_items() -> Result<Vec<ShoppingItem>> {
        let db = Self::load()?;
        Ok(db.shopping_list)
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
// ADD SHOPPING LOIST ITEM
// ----------------------------------------------
impl Storage {
    pub fn add_shopping_item(item: ShoppingItem) -> Result<()> {
        let mut db = Self::load()?;

        // Generiere eine eindeutige ID
        let new_id = db.shopping_list.iter().map(|i| i.id).max().unwrap_or(0) + 1;

        let mut item = item;
        item.id = new_id;
        item.purchased = false;

        db.shopping_list.push(item);
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

// ----------------------------------------------
// DEFAULT ARTICLES
// ----------------------------------------------

use serde::Deserialize;
use std::collections::HashMap;

const DEFAULT_ARTICLES_TOML: &str = r#"
[categories]
"Obst und Gemüse" = [
    "Äpfel", "Bananen", "Orangen", "Birnen", "Beeren (Erdbeeren, Himbeeren, Blaubeeren etc.)",
    "Trauben", "Melone (Wassermelone, Cantaloupe etc.)", "Zitronen", "Limetten", "Tomaten",
    "Gurken", "Salat (verschiedene Sorten)", "Paprika (verschiedene Farben)", "Zwiebeln",
    "Knoblauch", "Karotten", "Kartoffeln", "Süßkartoffeln", "Brokkoli", "Blumenkohl",
    "Zucchini", "Auberginen", "Spinat", "Grünkohl", "Erbsen", "Bohnen", "Mais", "Avocado",
    "Pilze", "Ingwer", "Knollen (Rote Bete, Sellerie etc.)"
]
"Getreideprodukte und Beilagen" = [
    "Brot (verschiedene Sorten)", "Brötchen", "Reis (verschiedene Sorten)",
    "Nudeln (verschiedene Sorten)", "Haferflocken", "Müsli", "Knäckebrot",
    "Kartoffelprodukte (Pommes, Kroketten etc.)", "Quinoa", "Couscous", "Bulgur"
]
"Milchprodukte und Alternativen" = [
    "Milch (Kuhmilch, Sojamilch, Mandelmilch etc.)", "Joghurt (verschiedene Sorten)",
    "Quark", "Käse (verschiedene Sorten)", "Sahne", "Butter",
    "vegane Alternativen (z.B. Tofu, Tempeh)"
]
"Fleisch, Fisch und Alternativen" = [
    "Hähnchen", "Rindfleisch", "Schweinefleisch",
    "Fisch (Lachs, Thunfisch, Kabeljau etc.)", "Wurstwaren", "Hackfleisch",
    "Eier", "Tofu", "Tempeh", "Veggie-Burger"
]
"Hülsenfrüchte, Nüsse und Samen" = [
    "Linsen", "Kichererbsen", "Bohnen (verschiedene Sorten)", "Erdnüsse",
    "Mandeln", "Cashewnüsse", "Walnüsse", "Sonnenblumenkerne", "Kürbiskerne", "Chiasamen"
]
"Fatte, Öle und Saucen" = [
    "Olivenöl", "Rapsöl", "Butter", "Mayonnaise", "Ketchup", "Senf", "Essig",
    "Sojasauce", "Kräuter und Gewürze (verschiedene Sorten)"
]
"Getränke" = [
    "Wasser", "Saft (verschiedene Sorten)", "Softdrinks", "Kaffee", "Tee",
    "Milch (s.o.)", "Alkohol (Bier, Wein, etc. - in Maßen)"
]
"Sonstiges" = [
    "Zucker", "Salz", "Honig", "Marmelade", "Schokolade", "Kekse", "Süßigkeiten",
    "Knabbereien (Chips, Salzstangen etc.)", "Fertiggerichte (Suppen, Konserven)",
    "Tiefkühlprodukte (Gemüse, Pizza, etc.)", "Brotbeläge (Wurst, Käse, Marmelade, etc.)",
    "Aufstriche (Pesto, Hummus, etc.)", "Eis", "Kuchen", "Backzutaten (Mehl, Backpulver, etc.)"
]
"#;

#[derive(Deserialize)]
struct DefaultArticles {
    categories: HashMap<String, Vec<String>>,
}

impl Storage {
    pub fn get_default_articles_path() -> Result<std::path::PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "dina", "dina-cli")
            .context("Could not determine project directory")?;

        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;

        Ok(data_dir.join("default_articles.toml"))
    }

    pub fn load_default_articles() -> Result<HashMap<String, Vec<String>>> {
        let path = Self::get_default_articles_path()?;

        // Create default file if it doesn't exist
        if !path.exists() {
            std::fs::write(&path, DEFAULT_ARTICLES_TOML)?;
        }

        let content = std::fs::read_to_string(&path)?;
        let default_articles: DefaultArticles = toml::from_str(&content)?;

        Ok(default_articles.categories)
    }
}
