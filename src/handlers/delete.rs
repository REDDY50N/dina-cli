use crate::cli::DeleteEntity;
use crate::storage;
use anyhow::Result;

pub fn handle_delete(entity: DeleteEntity) -> Result<()> {
    match entity {
        DeleteEntity::Recipe { name } => handle_delete_recipe(&name),
        DeleteEntity::Article { name } => handle_delete_article(&name),
        DeleteEntity::Meal { day } => handle_delete_meal(&day),
        DeleteEntity::Shopping { id } => handle_delete_shopping(id),
    }
}

fn handle_delete_recipe(name: &str) -> Result<()> {
    storage::Storage::delete_recipe(name)?;
    println!("Delete recipe: {}", name);
    Ok(())
}

fn handle_delete_article(name: &str) -> Result<()> {
    storage::Storage::delete_article(name)?;
    println!("Delete article: {}", name);
    Ok(())
}

fn handle_delete_meal(day: &str) -> Result<()> {
    storage::Storage::delete_meal(day)?;
    println!("Delete Meal for {}", day);
    Ok(())
}

fn handle_delete_shopping(id: usize) -> Result<()> {
    storage::Storage::delete_shopping_item(id)?;
    println!("Einkaufslisten-Item mit ID {} gelöscht", id);
    Ok(())
}
