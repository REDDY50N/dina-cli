mod cli;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};
use models::{Article, MealPlan, Recipe, ShoppingItem};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { entity } => handle_list(entity),
        Commands::Show { entity } => println!("Show command would be executed"),
        Commands::Add { entity } => println!("Add command would be executed"),
        Commands::Edit { entity } => println!("Edit command would be executed"),
        Commands::Delete { entity } => println!("Delete command would be executed"),
    }
}

fn handle_list(entity: cli::ListEntity) {
    match entity {
        cli::ListEntity::Recipes => println!("Listing all recipes"),
        cli::ListEntity::Articles => println!("Listing all articles"),
        cli::ListEntity::Meals => println!("Listing all meals"),
        cli::ListEntity::Shopping => println!("Listing shopping list"),
    }
}



/*
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { entity } => handle_list(entity),
        Commands::Show { entity } => handle_show(entity),
        Commands::Add { entity } => handle_add(entity),
        Commands::Edit { entity } => handle_edit(entity),
        Commands::Delete { entity } => handle_delete(entity),
    }
}

// Implementierung der Handler-Funktionen
// -- List
fn handle_list(entity: cli::ListEntity) {
    match entity {
        cli::ListEntity::Recipes => storage::list_recipes(),
        cli::ListEntity::Articles => storage::list_articles(),
        cli::ListEntity::Meals => storage::list_meals(),
        cli::ListEntity::Shopping => storage::list_shopping(),
    }
}

// -- Show
fn handle_show(entity: cli::ListEntity) {
    match entity {
        cli::ListEntity::Recipes => storage::show_recipes(),
        cli::ListEntity::Articles => storage::list_articles(),
        cli::ListEntity::Meals => storage::list_meals(),
        cli::ListEntity::Shopping => storage::list_shopping(),
    }
}

// -- Edit
fn handle_show(entity: cli::ListEntity) {
    match entity {
        cli::ListEntity::Recipes => storage::list_recipes(),
        cli::ListEntity::Articles => storage::list_articles(),
        cli::ListEntity::Meals => storage::list_meals(),
        cli::ListEntity::Shopping => storage::list_shopping(),
    }
}
*/