use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dina")]
#[command(version = "1.0")]
#[command(about = "Alltagshelfer für Essensplanung", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

// ----------------------------------------------
// COMMANDS
// ----------------------------------------------
#[derive(Subcommand)]
pub enum Commands {
    /// Listet Elemente (Rezepte, Artikel, Mahlzeiten, Einkaufsliste)
    List {
        #[command(subcommand)]
        entity: ListEntity,
    },

    /// Zeigt Details eines Elements
    Show {
        #[command(subcommand)]
        entity: ShowEntity,
    },

    /// Fügt neues Element hinzu
    Add {
        #[command(subcommand)]
        entity: AddEntity,
    },

    /// Bearbeitet vorhandenes Element
    Edit {
        #[command(subcommand)]
        entity: EditEntity,
    },

    /// Löscht Element
    Delete {
        #[command(subcommand)]
        entity: DeleteEntity,
    },

    /// Synchronisiert die Datenbank mit GitHub
    Sync {
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
}

// ----------------------------------------------
// SHOW CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum ShowEntity {
    Recipe { name: String },
    Article { name: String },
}

// ----------------------------------------------
// ADD CMD
// ----------------------------------------------
#[derive(Subcommand)]
pub enum AddEntity {
    Recipe {
        name: String,
        content: String,
    },
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
    Meal {
        day: String,
        recipe: String,
    },
    Shopping {
        item: String,
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
    Recipe { name: String },
    Article { name: String },
    Meal { day: String },
}

// ----------------------------------------------
// SYNC WITH REMOTE GIT
// ----------------------------------------------
#[derive(Subcommand)]
pub enum SyncAction {
    /// Holt die neueste Version der Datenbank von GitHub
    Pull,

    /// Sendet lokale Änderungen an GitHub
    Push {
        #[arg(short, long)]
        message: Option<String>,
    },

    /// Setzt den GitHub Personal Access Token
    SetToken { token: String },
}
