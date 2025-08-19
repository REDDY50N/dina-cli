use crate::cli::ListEntity;
use crate::storage;
use anyhow::Result;

pub fn handle_list(entity: ListEntity) -> Result<()> {
    match entity {
        ListEntity::Recipes => handle_list_recipes(),
        ListEntity::Articles => handle_list_articles(),
        ListEntity::Meals => handle_list_meals(),
        ListEntity::Shopping => handle_list_shopping(),
    }
}

fn handle_list_recipes() -> Result<()> {
    println!("Listing all recipes:");
    // Hier später die Rezepte auflisten
    Ok(())
}

fn handle_list_articles() -> Result<()> {
    let articles = storage::Storage::list_articles()?;
    if articles.is_empty() {
        println!("Keine Artikel im Lager.");
    } else {
        println!(
            "{:<20} {:<10} {:<10} {:<15} {:<10}",
            "Name", "Menge", "Einheit", "Kategorie", "Min-Bestand"
        );
        println!("{}", "-".repeat(65));

        for article in articles {
            println!(
                "{:<20} {:<10} {:<10} {:<15} {:<10}",
                article.name,
                article.quantity,
                article.unit.as_deref().unwrap_or("-"),
                article.category.as_deref().unwrap_or("-"),
                article
                    .min_quantity
                    .map_or("-".to_string(), |q| q.to_string())
            );
        }
    }
    Ok(())
}

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

fn handle_list_shopping() -> Result<()> {
    let items = storage::Storage::list_shopping_items()?;
    if items.is_empty() {
        println!("Einkaufsliste ist leer.");
    } else {
        println!(
            "{:<5} {:<20} {:<10} {:<10} {:<15} {:<10}",
            "ID", "Artikel", "Menge", "Einheit", "Laden", "Erledigt"
        );
        println!("{}", "-".repeat(75));

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
    }
    Ok(())
}
