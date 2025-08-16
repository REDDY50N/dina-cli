use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dina")]
#[command(version = "1.0")]
#[command(about = "Alltagshelfer für Essensplanung", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

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
}

#[derive(Subcommand)]
pub enum ListEntity {
    Recipes,
    Articles,
    Meals,
    Shopping,
}

#[derive(Subcommand)]
pub enum ShowEntity {
    Recipe { name: String },
    Article { name: String },
}

#[derive(Subcommand)]
pub enum AddEntity {
    Recipe { name: String, content: String },
    Article { name: String, quantity: u32 },
    Meal { day: String, recipe: String },
    Shopping { item: String },
}

#[derive(Subcommand)]
pub enum EditEntity {
    Recipe { name: String },
    Article { name: String },
    Meal { day: String },
    Shopping { id: usize },
}

#[derive(Subcommand)]
pub enum DeleteEntity {
    Recipe { name: String },
    Article { name: String },
    Meal { day: String },
}

/*
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dina")]
#[command(version = "1.0")]
#[command(about = "Alltagshelfer für Essensplanung", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

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
}

#[derive(Subcommand)]
pub enum ListEntity {
    Recipes,
    Articles,
    Meals,
    Shopping,
}

#[derive(Subcommand)]
pub enum ShowEntity {
    Recipe { name: String },
    Article { name: String },
}

#[derive(Subcommand)]
pub enum AddEntity {
    Recipe { name: String, content: String },
    Article { name: String, quantity: u32 },
    Meal { day: String, recipe: String },
    Shopping { item: String },
}

#[derive(Subcommand)]
pub enum EditEntity {
    Recipe { name: String },
    Article { name: String },
    Meal { day: String },
    Shopping { id: usize },
}

#[derive(Subcommand)]
pub enum DeleteEntity {
    Recipe { name: String },
    Article { name: String },
    Meal { day: String },
}
*/
