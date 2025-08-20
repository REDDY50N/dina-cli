use crate::models::FoodItem;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;

const DEFAULT_NUTRIENTS_TOML: &str = r#"
# Standard-Nährwertdatenbank
# Alle Werte sind pro 100g des Lebensmittels

[[foods]]
name = "Äpfel"
category = "Obst und Gemüse"
nutrients = { protein = 0.3, carbohydrates = 14.0, sugar = 10.0, fat = 0.2, saturated_fat = 0.0, salt = 0.0, calories = 52 }

[[foods]]
name = "Bananen"
category = "Obst und Gemüse"
nutrients = { protein = 1.1, carbohydrates = 22.8, sugar = 12.2, fat = 0.3, saturated_fat = 0.1, salt = 0.0, calories = 89 }

[[foods]]
name = "Vollkornbrot"
category = "Getreideprodukte und Beilagen"
nutrients = { protein = 7.5, carbohydrates = 41.0, sugar = 2.5, fat = 1.8, saturated_fat = 0.4, salt = 1.1, calories = 220 }

[[foods]]
name = "Lachs"
category = "Fleisch, Fisch und Alternativen"
nutrients = { protein = 20.0, carbohydrates = 0.0, sugar = 0.0, fat = 13.0, saturated_fat = 3.0, salt = 0.1, calories = 208 }

[[foods]]
name = "Tofu"
category = "Fleisch, Fisch und Alternativen"
nutrients = { protein = 8.0, carbohydrates = 1.9, sugar = 0.0, fat = 4.8, saturated_fat = 0.7, salt = 0.0, calories = 76 }

[[foods]]
name = "Joghurt"
category = "Milchprodukte und Alternativen"
nutrients = { protein = 3.5, carbohydrates = 4.7, sugar = 4.7, fat = 3.3, saturated_fat = 2.1, salt = 0.1, calories = 61 }

[[foods]]
name = "Olivenöl"
category = "Fette, Öle und Saucen"
nutrients = { protein = 0.0, carbohydrates = 0.0, sugar = 0.0, fat = 100.0, saturated_fat = 14.0, salt = 0.0, calories = 884 }

# Weitere Lebensmittel hier hinzufügen...
"#;

#[derive(Debug, Serialize, Deserialize)]
struct NutrientDatabase {
    foods: Vec<FoodItem>,
}

pub struct NutrientStorage;

impl NutrientStorage {
    pub fn get_db_path() -> Result<std::path::PathBuf> {
        let proj_dirs = directories::ProjectDirs::from("com", "dina", "dina-cli")
            .context("Could not determine project directory")?;

        let data_dir = proj_dirs.data_dir();
        std::fs::create_dir_all(data_dir)?;

        Ok(data_dir.join("nutrients.toml"))
    }

    pub fn load_nutrients() -> Result<Vec<FoodItem>> {
        let path = Self::get_db_path()?;

        if !path.exists() {
            std::fs::write(&path, DEFAULT_NUTRIENTS_TOML)?;
        }

        let content = std::fs::read_to_string(&path)?;
        let db: NutrientDatabase = toml::from_str(&content)?;

        Ok(db.foods)
    }

    pub fn find_food(name: &str) -> Result<Option<FoodItem>> {
        let foods = Self::load_nutrients()?;
        let food = foods
            .into_iter()
            .find(|f| f.name.to_lowercase() == name.to_lowercase());

        Ok(food)
    }

    pub fn get_foods_by_nutrient(nutrient_type: &str) -> Result<Vec<FoodItem>> {
        let mut foods = Self::load_nutrients()?;

        match nutrient_type.to_lowercase().as_str() {
            "protein" => foods.sort_by(|a, b| {
                b.nutrients
                    .protein
                    .partial_cmp(&a.nutrients.protein)
                    .unwrap()
            }),
            "carbohydrates" => foods.sort_by(|a, b| {
                b.nutrients
                    .carbohydrates
                    .partial_cmp(&a.nutrients.carbohydrates)
                    .unwrap()
            }),
            "sugar" => {
                foods.sort_by(|a, b| b.nutrients.sugar.partial_cmp(&a.nutrients.sugar).unwrap())
            }
            "fat" => foods.sort_by(|a, b| b.nutrients.fat.partial_cmp(&a.nutrients.fat).unwrap()),
            "saturated_fat" => foods.sort_by(|a, b| {
                b.nutrients
                    .saturated_fat
                    .partial_cmp(&a.nutrients.saturated_fat)
                    .unwrap()
            }),
            "salt" => {
                foods.sort_by(|a, b| b.nutrients.salt.partial_cmp(&a.nutrients.salt).unwrap())
            }
            "calories" => foods.sort_by(|a, b| {
                let a_cal = a.nutrients.calories.unwrap_or(0);
                let b_cal = b.nutrients.calories.unwrap_or(0);
                b_cal.cmp(&a_cal)
            }),
            _ => anyhow::bail!(
                "Unbekannter Nährstofftyp: {}. Erlaubt sind: protein, carbohydrates, sugar, fat, saturated_fat, salt, calories",
                nutrient_type
            ),
        }

        Ok(foods)
    }
}
