use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "foody")]
#[command(version = "1.0")]
#[command(about = "Daily Assist (Meal Planer, Shopping List, Food Stock)", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// ----------------------------------------------
// DINA COMMANDS
// ----------------------------------------------
#[derive(Subcommand)]
pub enum Commands {
    // -- List elements (recipes, articles, meals, shopping list)
    List {
        #[command(subcommand)]
        entity: ListEntity,
    },

    // -- Shows element details
    Show {
        #[command(subcommand)]
        entity: ShowEntity,
    },

    // -- Add new element
    Add {
        #[command(subcommand)]
        entity: AddEntity,
    },

    // -- Edit existing element
    Edit {
        #[command(subcommand)]
        entity: EditEntity,
    },

    // -- Delete existing element
    Delete {
        #[command(subcommand)]
        entity: DeleteEntity,
    },

    // -- Sync with remote git repo
    Git {
        #[command(subcommand)]
        action: SyncAction,
    },
}

// ----------------------------------------------
// LIST CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum ListEntity {
    Recipes,
    Articles,
    Meals,
    Shopping,
    DefaultArticles, //#[command(name = "default-articles")]

    // Nutrients {
    //     #[arg(short, long)]
    //     nutrient_type: Option<String>,
    // },
}

// ----------------------------------------------
// SHOW CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum ShowEntity {
    Recipe { name: String },
    Article { name: String },
    // Nutrient { name: String },
}

// ----------------------------------------------
// ADD CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum AddEntity {
    // -- RECIPE
    Recipe {
        name: String,
        content: String,
    },

    // -- FOOD STOCK
    Article {
        name: String,
        quantity: u32,
        #[arg(short, long)]
        unit: Option<String>,
        #[arg(short, long)]
        category: Option<String>,
        #[arg(long)]
        min_quantity: Option<u32>,
    },

    // -- MEAL PLAN
    Meal {
        day: String,
        recipe: String,
    },

    // -- SHPPING LIST
    Shopping {
        item: String,
        quantity: u32,
        #[arg(short, long)]
        unit: Option<String>,
        #[arg(short, long)]
        store: Option<String>,
    },
}

// ----------------------------------------------
// EDIT CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum EditEntity {
    Recipe { name: String },
    Article { name: String },
    Meal { day: String },
    Shopping { id: usize },
}

// ----------------------------------------------
// DELETE CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum DeleteEntity {
    Recipe { name: String },  // Delete recipe by name
    Article { name: String }, // Delete article by name
    Meal { day: String },     // Delete meal by day (monday, tuesday, ...)
    Shopping { id: usize },   // Delete shopping list item by ID
}

// ----------------------------------------------
// SYNC CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum SyncAction {
    /// Initialisiert das Datenverzeichnis als Git-Repository
    Init { remote_url: String },
    /// Holt die neueste Version der Datenbank vom Remote-Repository
    Pull,
    /// Sendet lokale Änderungen an das Remote-Repository
    Push {
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Zeigt den aktuellen Synchronisationsstatus
    Status,
}
