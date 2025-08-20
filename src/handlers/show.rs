use crate::cli::ShowEntity;
use anyhow::Result;

use crate::nutrients::NutrientStorage;

// ----------------------------------------------
// MATCH CASE
// ----------------------------------------------
pub fn handle_show(entity: ShowEntity) -> Result<()> {
    match entity {
        ShowEntity::Recipe { name } => {
            println!("Showing recipe: {}", name);
            Ok(())
        }
        ShowEntity::Article { name } => {
            println!("Showing article: {}", name);
            Ok(())
        }

        ShowEntity::Nutrient { name } => handle_show_nutrient(&name),
    }
}

// ----------------------------------------------
// SHOW NUTRIENT FOR <FOOD>
// ----------------------------------------------
fn handle_show_nutrient(name: &str) -> anyhow::Result<()> {
    if let Some(food) = NutrientStorage::find_food(name)? {
        println!("Nährwertangaben für {} (pro 100g):", food.name);
        println!("{}", "=".repeat(40));
        println!("Kategorie: {}", food.category);
        println!("Protein: {:.1}g", food.nutrients.protein);
        println!("Kohlenhydrate: {:.1}g", food.nutrients.carbohydrates);
        println!("  davon Zucker: {:.1}g", food.nutrients.sugar);
        println!("Fett: {:.1}g", food.nutrients.fat);
        println!(
            "  davon gesättigte Fettsäuren: {:.1}g",
            food.nutrients.saturated_fat
        );
        println!("Salz: {:.1}g", food.nutrients.salt);

        if let Some(calories) = food.nutrients.calories {
            println!("Kalorien: {} kcal", calories);
        }
    } else {
        anyhow::bail!("Keine Nährwertdaten für '{}' gefunden", name);
    }

    Ok(())
}

// ----------------------------------------------
// SHOW NUTRIENT FOR <FOOD>
// ----------------------------------------------
