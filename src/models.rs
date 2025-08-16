use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub name: String,
    pub quantity: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MealPlan {
    pub day: String,
    pub recipe: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShoppingItem {
    pub id: usize,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub recipes: Vec<Recipe>,
    pub articles: Vec<Article>,
    pub meal_plans: Vec<MealPlan>,
    pub shopping_list: Vec<ShoppingItem>,
}

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
