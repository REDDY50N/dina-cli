mod cli;
mod git_sync;
mod models;
mod storage;

use clap::Parser;

use cli::{Cli, Commands, SyncAction};
use models::{Article, MealPlan, Recipe, ShoppingItem};

// ----------------------------------------------
// -- MAIN FUNCTION
// ----------------------------------------------
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

// ----------------------------------------------
// RUN FUNCTION
// ----------------------------------------------
fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { entity } => handle_list(entity),
        Commands::Show { entity } => handle_show(entity),
        Commands::Add { entity } => handle_add(entity),
        Commands::Edit { entity } => handle_edit(entity),
        Commands::Delete { entity } => handle_delete(entity),
        Commands::Git { action } => handle_sync(action),
    }
}

// ----------------------------------------------
// HADNLE - LIST
// ----------------------------------------------
fn handle_list(entity: cli::ListEntity) -> anyhow::Result<()> {
    match entity {
        // -- LIST RECIPES
        cli::ListEntity::Recipes => {
            println!("Listing all recipes:");
            // Hier später die Rezepte auflisten
            Ok(())
        }

        // -- LIST ARTICLES
        cli::ListEntity::Articles => {
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

        // -- MEAL PLAN
        cli::ListEntity::Meals => {
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

        // -- SHOPPING LIST
        cli::ListEntity::Shopping => {
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
    }
}

// ----------------------------------------------
// HADNLE - SHOW
// ----------------------------------------------
fn handle_show(entity: cli::ShowEntity) -> anyhow::Result<()> {
    match entity {
        cli::ShowEntity::Recipe { name } => println!("Showing recipe: {}", name),
        cli::ShowEntity::Article { name } => println!("Showing article: {}", name),
    }
    Ok(())
}

// ----------------------------------------------
// HADNLE - ADD
// ----------------------------------------------
fn handle_add(entity: cli::AddEntity) -> anyhow::Result<()> {
    match entity {
        // -- ADD RECIPE
        cli::AddEntity::Recipe { name, content } => {
            let recipe = Recipe { name, content };
            storage::Storage::add_recipe(recipe)?;
            println!("Rezept erfolgreich hinzugefügt!");
            Ok(())
        }

        // -- ADD ARTICLE TO ERP STORAGE
        cli::AddEntity::Article {
            name,
            quantity,
            unit,
            category,
            min_quantity,
        } => {
            let article = Article {
                name,
                quantity,
                unit,
                category,
                min_quantity,
            };
            storage::Storage::add_article(article)?;
            println!("Artikel erfolgreich hinzugefügt/aktualisiert!");
            Ok(())
        }

        // -- ADD MEAL TO MEAL PLAN
        cli::AddEntity::Meal { day, recipe } => {
            // Validiere Tag
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
            println!("Add meal {:#?} to plan.", meal);
            storage::Storage::add_meal(meal)?;
            Ok(())
        }

        // -- ADD SHOPPING LIST ITEM
        cli::AddEntity::Shopping {
            item,
            quantity,
            unit,
            store,
        } => {
            let shopping_item = ShoppingItem {
                id: 0, // Wird in add_shopping_item überschrieben
                name: item,
                quantity,
                unit,
                store,
                purchased: false,
            };
            println!("Add Item to shopping list: {:#?}", shopping_item);
            storage::Storage::add_shopping_item(shopping_item)?;
            Ok(())
        }
    }
}

// ----------------------------------------------
// HADNLE - EDIT
// ----------------------------------------------

fn handle_edit(entity: cli::EditEntity) -> anyhow::Result<()> {
    match entity {
        cli::EditEntity::Recipe { name } => println!("Editing recipe: {}", name),
        cli::EditEntity::Article { name } => println!("Editing article: {}", name),
        cli::EditEntity::Meal { day } => println!("Editing meal on: {}", day),
        cli::EditEntity::Shopping { id } => println!("Editing shopping item: {}", id),
    }
    Ok(())
}

// ----------------------------------------------
// HADNLE - DELETE
// ----------------------------------------------
fn handle_delete(entity: cli::DeleteEntity) -> anyhow::Result<()> {
    match entity {
        cli::DeleteEntity::Recipe { name } => {
            storage::Storage::delete_recipe(&name)?;
            println!("Delete recipe: {}", name);
            Ok(())
        }
        cli::DeleteEntity::Article { name } => {
            storage::Storage::delete_article(&name)?;
            println!("Delete article: {}", name);
            Ok(())
        }
        // TODO: Add meal name
        cli::DeleteEntity::Meal { day } => {
            storage::Storage::delete_meal(&day)?;
            println!("Delete Meal for {}", day);
            Ok(())
        }
        // TODO: Add articel name
        cli::DeleteEntity::Shopping { id } => {
            storage::Storage::delete_shopping_item(id)?;
            println!("Einkaufslisten-Item mit ID {} gelöscht", id);
            Ok(())
        }
    }
}

// ----------------------------------------------
// HADNLE - GIT SYNC
// ----------------------------------------------
fn handle_sync(action: SyncAction) -> anyhow::Result<()> {
    match action {
        // -- INIT
        SyncAction::Init { remote_url } => {
            git_sync::GitSync::init(&remote_url)?;
            println!("Repository initialized with remote: {}", remote_url);
            Ok(())
        }
        // -- PULL
        SyncAction::Pull => {
            git_sync::GitSync::pull()?;
            Ok(())
        }
        // -- PUSH
        SyncAction::Push { message } => {
            let commit_message = message.unwrap_or_else(|| "Update from dina-cli".to_string());
            git_sync::GitSync::push(&commit_message)
        }
        // -- STATUS
        SyncAction::Status => {
            git_sync::GitSync::status()?;
            Ok(())
        }
    }
}
