use crate::cli::ShowEntity;
use anyhow::Result;

pub fn handle_show(entity: ShowEntity) -> Result<()> {
    match entity {
        ShowEntity::Recipe { name } => {
            println!("Showing recipe: {}", name);
            Ok(())
        }
        ShowEntity::Article { name } => {
            println!("Showing article: {}", name);
            Ok(())
        }
    }
}
