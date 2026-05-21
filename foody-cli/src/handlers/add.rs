use crate::cli::AddEntity;
use foody_core::models::{Article, MealPlan, ShoppingItem};
use foody_core::recipe::RecipeStore;

use crate::storage;

use anyhow::Result;

// ----------------------------------------------
// MATCH CASE
// ----------------------------------------------
pub fn handle_add(entity: AddEntity) -> Result<()> {
    match entity {
        AddEntity::Recipe { name, content } => handle_add_recipe(name, content),
        AddEntity::Article {
            name,
            quantity,
            unit,
            category,
            min_quantity,
        } => handle_add_article(name, quantity, unit, category, min_quantity),
        AddEntity::Meal { day, recipe } => handle_add_meal(day, recipe),
        AddEntity::Shopping {
            item,
            quantity,
            unit,
            store,
        } => handle_add_shopping(item, quantity, unit, store),
    }
}

// ----------------------------------------------
// RECIPE
// ----------------------------------------------
// fn handle_add_recipe(name: String, content: String) -> Result<()> {
//     let recipe = Recipe { name, content };
//     storage::Storage::add_recipe(recipe)?;
//     println!("Rezept erfolgreich hinzugefügt!");
//     Ok(())
// }

fn handle_add_recipe(name: String, content: String) -> Result<()> {
    let recipe_store = RecipeStore::new()?;
    recipe_store.save_recipe(&name, &content)?;
    println!("Add recipe: {}", name);
    Ok(())
}

// ----------------------------------------------
// ARTICLE
// ----------------------------------------------
fn handle_add_article(
    name: String,
    quantity: u32,
    unit: Option<String>,
    category: Option<String>,
    min_quantity: Option<u32>,
) -> Result<()> {
    let article = Article {
        name,
        quantity,
        unit,
        category,
        min_quantity,
    };
    println!("Add/update article: {}", article.name);
    storage::Storage::add_article(article)?;
    Ok(())
}

// ----------------------------------------------
// MEAL (MEAL PLAN)
// ----------------------------------------------
fn handle_add_meal(day: String, recipe: String) -> Result<()> {
    let valid_days = [
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];

    if !valid_days.contains(&day.to_lowercase().as_str()) {
        anyhow::bail!(
            "Ungültiger Wochentag: {}. Erlaubt sind: {}",
            day,
            valid_days.join(", ")
        );
    }

    let meal = MealPlan { day, recipe };
    println!("Add meal to meal plan: {} Day: {}", meal.recipe, meal.day);
    storage::Storage::add_meal(meal)?;
    Ok(())
}

// ----------------------------------------------
// SHOPPING LIST ITEM
// ----------------------------------------------
fn handle_add_shopping(
    item: String,
    quantity: u32,
    unit: Option<String>,
    store: Option<String>,
) -> Result<()> {
    let shopping_item = ShoppingItem {
        id: 0,
        name: item,
        quantity,
        unit,
        store,
        purchased: false,
    };
    println!("Add item to shopping list: {}", shopping_item.name);
    storage::Storage::add_shopping_item(shopping_item)?;
    Ok(())
}
