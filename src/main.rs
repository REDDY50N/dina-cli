mod cli;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use models::Article;
use models::Recipe;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { entity } => handle_list(entity),
        Commands::Show { entity } => handle_show(entity),
        Commands::Add { entity } => handle_add(entity)?,
        Commands::Edit { entity } => handle_edit(entity),
        Commands::Delete { entity } => handle_delete(entity),
    }

    Ok(())
}

fn handle_list(entity: cli::ListEntity) {
    match entity {
        cli::ListEntity::Recipes => println!("Listing all recipes"),
        cli::ListEntity::Articles => println!("Listing all articles"),
        cli::ListEntity::Meals => println!("Listing all meals"),
        cli::ListEntity::Shopping => println!("Listing shopping list"),
    }
}

fn handle_show(entity: cli::ShowEntity) {
    match entity {
        cli::ShowEntity::Recipe { name } => println!("Showing recipe: {}", name),
        cli::ShowEntity::Article { name } => println!("Showing article: {}", name),
    }
}

fn handle_add(entity: cli::AddEntity) -> anyhow::Result<()> {
    match entity {
        cli::AddEntity::Recipe { name, content } => {
            let recipe = Recipe { name, content };
            storage::Storage::add_recipe(recipe)?;
            println!("Recipe added successfully!");
            Ok(())
        }

        cli::AddEntity::Article { name, quantity } => {
            let article = Article { name, quantity };
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

fn handle_edit(entity: cli::EditEntity) {
    match entity {
        cli::EditEntity::Recipe { name } => println!("Editing recipe: {}", name),
        cli::EditEntity::Article { name } => println!("Editing article: {}", name),
        cli::EditEntity::Meal { day } => println!("Editing meal on: {}", day),
        cli::EditEntity::Shopping { id } => println!("Editing shopping item: {}", id),
    }
}

fn handle_delete(entity: cli::DeleteEntity) {
    match entity {
        cli::DeleteEntity::Recipe { name } => println!("Deleting recipe: {}", name),
        cli::DeleteEntity::Article { name } => println!("Deleting article: {}", name),
        cli::DeleteEntity::Meal { day } => println!("Deleting meal on: {}", day),
    }
}
