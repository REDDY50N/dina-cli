use crate::cli::ListEntity;
//use foody_core::nutrients::NutrientStorage;
use foody_core::recipe::RecipeStore;
use crate::storage;
use anyhow::Result;

// ----------------------------------------------
// MATCH CASES
// ----------------------------------------------
// HINT: Case must match a function below
pub fn handle_list(entity: ListEntity) -> Result<()> {
    match entity {
        ListEntity::Recipes => handle_list_recipes(),
        ListEntity::Articles => handle_list_articles(),
        ListEntity::Meals => handle_list_meals(),
        ListEntity::Shopping => handle_list_shopping(),
        ListEntity::DefaultArticles => handle_list_default_articles(),
        // ListEntity::Nutrients { nutrient_type } => handle_list_nutrients(nutrient_type),
    }
}

// ----------------------------------------------
// LIST RECIPES
// ----------------------------------------------
fn handle_list_recipes() -> Result<()> {
    let recipe_store = RecipeStore::new()?;
    let recipes = recipe_store.list_recipes()?;

    if recipes.is_empty() {
        println!("No recipes found! Recipe folder is empty!");
        println!("Use 'dina add recipe \"<Name>\" \"<Content>\"' to add a recipe.");
    } else {
        println!("══════════════════════════════════════════════════");
        println!("  RECIPES                                          ");
        println!("══════════════════════════════════════════════════");

        for recipe in recipes {
            println!(" • {}", recipe);
        }
        println!("══════════════════════════════════════════════════");
    }
    Ok(())
}

// ----------------------------------------------
// LIST ARTICLES
// ----------------------------------------------
fn handle_list_articles() -> Result<()> {
    let articles = storage::Storage::list_articles()?;
    if articles.is_empty() {
        println!("No article in stock. Storage is empty!");
    } else {
        println!(
            "{:<20} {:<10} {:<10} {:<15} {:<10}",
            "Product", "Quantity", "Unit", "Category", "Min. Stock"
        );
        println!("{}", "-".repeat(65));

        for article in articles {
            println!(
                "{:<20} {:<10} {:<10} {:<15} {:<10}",
                article.name,
                article.quantity,
                article.unit.as_deref().unwrap_or("-"),
                article.category.as_deref().unwrap_or("-"),
                article.min_quantity.map_or("-".to_string(), |q| q.to_string())
            );
        }
    }
    Ok(())
}

// ----------------------------------------------
// LIST MEALS
// ----------------------------------------------
fn handle_list_meals() -> Result<()> {
    let meals = storage::Storage::list_meals()?;

    const WEEKDAYS: [&str; 7] = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    let mut meal_map = std::collections::HashMap::new();
    for meal in &meals {
        meal_map.insert(meal.day.to_lowercase(), meal.recipe.clone());
    }

    println!("╔════════════╦═══════════════════════════════════════════════════╗");
    println!("║ {:10} ║ {:45}     ║", "Tag", "Gericht");
    println!("╠════════════╬═══════════════════════════════════════════════════╣");

    for day in WEEKDAYS {
        let recipe = meal_map
            .get(&day.to_lowercase())
            .map(|s| s.clone())
            .unwrap_or_else(|| "-".to_string());

        let display_recipe = if recipe.len() > 43 {
            format!("{}...", &recipe[..40])
        } else {
            recipe
        };

        println!("║ {:10} ║ {:45}     ║", day, display_recipe);
    }

    println!("╚════════════╩═══════════════════════════════════════════════════╝");
    Ok(())
}

// ----------------------------------------------
// LIST SHOPPING LIST ARTICLES
// ----------------------------------------------
fn handle_list_shopping() -> Result<()> {
    let items = storage::Storage::list_shopping_items()?;
    if items.is_empty() {
        println!("Shopping list is empty!");
    } else {
        println!("{}", "═".repeat(75)); // ASCII Line
        println!("{:^75}", "SHOPPING LIST"); // in the mid of 75
        println!("{}", "═".repeat(75)); // ASCII Line
        println!(
            "{:<5} {:<20} {:<10} {:<10} {:<15} {:<10}",
            "ID", "Product", "Quantity", "Unit", "Store/Brand", "Done"
        );
        println!("{}", "═".repeat(75)); // ASCII Line

        for item in items {
            let purchased = if item.purchased { "✓" } else { "" };
            println!(
                "{:<5} {:<20} {:<10} {:<10} {:<15} {:<10}",
                item.id,
                item.name,
                item.quantity,
                item.unit.as_deref().unwrap_or("-"),
                item.store.as_deref().unwrap_or("-"),
                purchased
            );
        }
        println!("{}", "═".repeat(75)); // ASCII Line
    }
    Ok(())
}

// ----------------------------------------------
// LIST DEFAULT ARTICLES
// ----------------------------------------------
// Articles are sorted by category in the toml file
fn handle_list_default_articles() -> anyhow::Result<()> {
    let categories = storage::Storage::load_default_articles()?;

    println!("Default Articles:");
    println!("{}", "═".repeat(50));

    for (category, items) in categories {
        println!("\n{}:", category);
        println!("{}", "-".repeat(category.len() + 1));

        for item in items {
            println!("  • {}", item);
        }
    }
    println!("{}", "═".repeat(50));

    println!("\nAdd to shopping list by: 'dina add shopping \"<Product>\" <Quantity>'");
    Ok(())
}

// ----------------------------------------------
// LIST NUTRIENTS
// ----------------------------------------------
// fn handle_list_nutrients(nutrient_type: Option<String>) -> anyhow::Result<()> {
//     if let Some(n_type) = nutrient_type {
//         let foods = NutrientStorage::get_foods_by_nutrient(&n_type)?;
//
//         println!("Lebensmittel sortiert nach {} (pro 100g):", n_type);
//         println!("{}", "=".repeat(60));
//         println!(
//             "{:<25} {:<8} {:<12} {:<8} {:<8} {:<8} {:<8} {:<8}",
//             "Name", "Protein", "Kohlenhyd.", "Zucker", "Fett", "ges. Fett", "Salz", "Kalorien"
//         );
//         println!("{}", "-".repeat(60));
//
//         for food in foods.iter().take(20) {
//             // Zeige Top 20
//             println!(
//                 "{:<25} {:<8.1} {:<12.1} {:<8.1} {:<8.1} {:<8.1} {:<8.1} {:<8}",
//                 food.name,
//                 food.nutrients.protein,
//                 food.nutrients.carbohydrates,
//                 food.nutrients.sugar,
//                 food.nutrients.fat,
//                 food.nutrients.saturated_fat,
//                 food.nutrients.salt,
//                 food.nutrients.calories.map_or("-".to_string(), |c| c.to_string())
//             );
//         }
//     } else {
//         let foods = NutrientStorage::load_nutrients()?;
//
//         println!("Alle Lebensmittel mit Nährwertangaben (pro 100g):");
//         println!("{}", "=".repeat(60));
//         println!(
//             "{:<25} {:<8} {:<12} {:<8} {:<8} {:<8} {:<8} {:<8}",
//             "Name", "Protein", "Kohlenhyd.", "Zucker", "Fett", "ges. Fett", "Salz", "Kalorien"
//         );
//         println!("{}", "-".repeat(60));
//
//         for food in foods {
//             println!(
//                 "{:<25} {:<8.1} {:<12.1} {:<8.1} {:<8.1} {:<8.1} {:<8.1} {:<8}",
//                 food.name,
//                 food.nutrients.protein,
//                 food.nutrients.carbohydrates,
//                 food.nutrients.sugar,
//                 food.nutrients.fat,
//                 food.nutrients.saturated_fat,
//                 food.nutrients.salt,
//                 food.nutrients.calories.map_or("-".to_string(), |c| c.to_string())
//             );
//         }
//     }
//
//     Ok(())
// }
