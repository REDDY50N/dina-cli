use crate::cli::EditEntity;
use anyhow::Result;

pub fn handle_edit(entity: EditEntity) -> Result<()> {
    match entity {
        EditEntity::Recipe { name } => {
            println!("Editing recipe: {}", name);
            Ok(())
        }
        EditEntity::Article { name } => {
            println!("Editing article: {}", name);
            Ok(())
        }
        EditEntity::Meal { day } => {
            println!("Editing meal on: {}", day);
            Ok(())
        }
        EditEntity::Shopping { id } => {
            println!("Editing shopping item: {}", id);
            Ok(())
        }
    }
}
