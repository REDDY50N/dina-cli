#[derive(Debug, Clone)]
pub struct Recipe {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub name: String,
    pub quantity: u32,
}

#[derive(Debug, Clone)]
pub struct MealPlan {
    pub day: String, // z.B. "Montag"
    pub recipe: String,
}

#[derive(Debug, Clone)]
pub struct ShoppingItem {
    pub id: usize,
    pub name: String,
}