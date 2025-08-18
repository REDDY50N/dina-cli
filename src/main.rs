mod cli;
mod git_sync;
mod models;
mod storage;

use clap::Parser;

use cli::{Cli, Commands, SyncAction};
use models::{Article, Recipe};

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
        Commands::Show { entity } => {
            handle_show(entity);
            Ok(())
        }
        Commands::Add { entity } => handle_add(entity),
        Commands::Edit { entity } => {
            handle_edit(entity);
            Ok(())
        }
        Commands::Delete { entity } => {
            handle_delete(entity);
            Ok(())
        }

        Commands::Git { action } => handle_sync(action),
    }
}

// ----------------------------------------------
// HADNLE - LIST
// ----------------------------------------------
fn handle_list(entity: cli::ListEntity) -> anyhow::Result<()> {
    match entity {
        cli::ListEntity::Recipes => {
            println!("Listing all recipes:");
            // Hier später die Rezepte auflisten
            Ok(())
        }
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
        cli::ListEntity::Meals => {
            println!("Listing all meals:");
            // Hier später die Mahlzeiten auflisten
            Ok(())
        }
        cli::ListEntity::Shopping => {
            println!("Listing shopping list:");
            // Hier später die Einkaufsliste auflisten
            Ok(())
        }
    }
}

// ----------------------------------------------
// HADNLE - SHOW
// ----------------------------------------------
fn handle_show(entity: cli::ShowEntity) {
    match entity {
        cli::ShowEntity::Recipe { name } => println!("Showing recipe: {}", name),
        cli::ShowEntity::Article { name } => println!("Showing article: {}", name),
    }
}

// ----------------------------------------------
// HADNLE - ADD
// ----------------------------------------------
fn handle_add(entity: cli::AddEntity) -> anyhow::Result<()> {
    match entity {
        cli::AddEntity::Recipe { name, content } => {
            let recipe = Recipe { name, content };
            storage::Storage::add_recipe(recipe)?;
            println!("Rezept erfolgreich hinzugefügt!");
            Ok(())
        }
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
        cli::AddEntity::Meal { day, recipe } => {
            println!("Adding meal: {} on {}", recipe, day);
            Ok(())
        }
        cli::AddEntity::Shopping { item } => {
            println!("Adding shopping item: {}", item);
            Ok(())
        }
    }
}

// ----------------------------------------------
// HADNLE - EDIT
// ----------------------------------------------
fn handle_edit(entity: cli::EditEntity) {
    match entity {
        cli::EditEntity::Recipe { name } => println!("Editing recipe: {}", name),
        cli::EditEntity::Article { name } => println!("Editing article: {}", name),
        cli::EditEntity::Meal { day } => println!("Editing meal on: {}", day),
        cli::EditEntity::Shopping { id } => println!("Editing shopping item: {}", id),
    }
}

// ----------------------------------------------
// HADNLE - DELETE
// ----------------------------------------------
fn handle_delete(entity: cli::DeleteEntity) {
    match entity {
        cli::DeleteEntity::Recipe { name } => println!("Deleting recipe: {}", name),
        cli::DeleteEntity::Article { name } => println!("Deleting article: {}", name),
        cli::DeleteEntity::Meal { day } => println!("Deleting meal on: {}", day),
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
