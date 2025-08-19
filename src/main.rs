mod cli;
mod git_sync;
mod handlers;
mod models;
mod storage;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { entity } => handlers::handle_list(entity),
        Commands::Show { entity } => handlers::handle_show(entity),
        Commands::Add { entity } => handlers::handle_add(entity),
        Commands::Edit { entity } => handlers::handle_edit(entity),
        Commands::Delete { entity } => handlers::handle_delete(entity),
        Commands::Git { action } => handlers::handle_sync(action),
    }
}
