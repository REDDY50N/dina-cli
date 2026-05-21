use crate::cli::ShowEntity;
use anyhow::Result;

// use foody_core::nutrients::NutrientStorage;
use foody_core::recipe::RecipeStore;

// ----------------------------------------------
// MATCH CASE
// ----------------------------------------------
pub fn handle_show(entity: ShowEntity) -> Result<()> {
    match entity {
        ShowEntity::Article { name } => {
            println!("Showing article: {}", name);
            Ok(())
        }

        ShowEntity::Recipe { name } => handle_show_recipe(&name),
        //ShowEntity::Nutrient { name } => handle_show_nutrient(&name),
    }
}

// ----------------------------------------------
// SHOW NUTRIENT FOR <FOOD>
// ----------------------------------------------
// fn handle_show_nutrient(name: &str) -> anyhow::Result<()> {
//     if let Some(food) = NutrientStorage::find_food(name)? {
//         println!("Nährwertangaben für {} (pro 100g):", food.name);
//         println!("{}", "=".repeat(40));
//         println!("Kategorie: {}", food.category);
//         println!("Protein: {:.1}g", food.nutrients.protein);
//         println!("Kohlenhydrate: {:.1}g", food.nutrients.carbohydrates);
//         println!("  davon Zucker: {:.1}g", food.nutrients.sugar);
//         println!("Fett: {:.1}g", food.nutrients.fat);
//         println!(
//             "  davon gesättigte Fettsäuren: {:.1}g",
//             food.nutrients.saturated_fat
//         );
//         println!("Salz: {:.1}g", food.nutrients.salt);
//
//         if let Some(calories) = food.nutrients.calories {
//             println!("Kalorien: {} kcal", calories);
//         }
//     } else {
//         anyhow::bail!("Keine Nährwertdaten für '{}' gefunden", name);
//     }
//
//     Ok(())
// }

// ----------------------------------------------
// RECIPE <NAME>
// ----------------------------------------------
pub fn handle_show_recipe(name: &str) -> Result<()> {
    let recipe_store = RecipeStore::new()?;
    let content = recipe_store.load_recipe(name)?;

    println!("Rezept: {}", name);
    println!("{}", "=".repeat(name.len() + 7));
    println!("{}", content);

    Ok(())
}

// ----------------------------------------------
// SHOW NUTRIENT FOR <FOOD>
// ----------------------------------------------
