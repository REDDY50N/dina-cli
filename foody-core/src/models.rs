use serde::{Deserialize, Serialize};

// ----------------------------------------------
// RECIPE
// ----------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub content: String,
}

// ----------------------------------------------
// ARTICLE
// ----------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub emoji: String,
    pub quantity: f64,
    pub unit: String,
    pub category: String,
    pub storage_location: String,
    pub min_quantity: Option<f64>,
    pub warning_active: bool,
}
// ----------------------------------------------
// WEEKLY MEAL PLAN
// ----------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealPlan {
    pub day: String,
    pub recipe: String,
}

// ----------------------------------------------
// SHOPPING LIST ITEM
// ----------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingItem {
    pub id: usize,
    pub name: String,
    pub quantity: u32,
    pub unit: Option<String>,
    pub store: Option<String>,
    pub purchased: bool,
}

// ----------------------------------------------
// NUTRIENT INFO
// ----------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NutrientInfo {
    pub protein: f32,          // g pro 100g
    pub carbohydrates: f32,    // g pro 100g
    pub sugar: f32,            // g pro 100g
    pub fat: f32,              // g pro 100g
    pub saturated_fat: f32,    // g pro 100g
    pub salt: f32,             // g pro 100g
    pub calories: Option<u32>, // kcal pro 100g (optional)
}

// ----------------------------------------------
// FOOD ITEM
// ----------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoodItem {
    pub name: String,
    pub category: String,
    pub nutrients: NutrientInfo,
}

// ----------------------------------------------
// DATABASE
// ----------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub recipes: Vec<Recipe>,
    pub articles: Vec<Article>,
    pub meal_plans: Vec<MealPlan>,
    pub shopping_list: Vec<ShoppingItem>,
}

// ----------------------------------------------
// DATABASE DEFAULT
// ----------------------------------------------
impl Default for Database {
    fn default() -> Self {
        Database {
            recipes: Vec::new(),
            articles: Vec::new(),
            meal_plans: Vec::new(),
            shopping_list: Vec::new(),
        }
    }
}
