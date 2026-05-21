use anyhow::{Result, anyhow};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct RecipeStore {
    recipe_dir: PathBuf,
}

impl RecipeStore {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "dina", "dina-cli")
            .ok_or_else(|| anyhow!("Could not determine project directory"))?;

        let recipe_dir = proj_dirs.data_dir().join("recipes");
        fs::create_dir_all(&recipe_dir)?;

        Ok(RecipeStore { recipe_dir })
    }

    pub fn get_recipe_path(&self, name: &str) -> PathBuf {
        let filename = format!("{}.md", sanitize_filename(name));
        self.recipe_dir.join(filename)
    }

    pub fn save_recipe(&self, name: &str, content: &str) -> Result<()> {
        let path = self.get_recipe_path(name);
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_recipe(&self, name: &str) -> Result<String> {
        let path = self.get_recipe_path(name);
        if !path.exists() {
            return Err(anyhow!("Rezept '{}' nicht gefunden", name));
        }

        let content = fs::read_to_string(path)?;
        Ok(content)
    }

    pub fn list_recipes(&self) -> Result<Vec<String>> {
        let mut recipes = Vec::new();

        for entry in fs::read_dir(&self.recipe_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    recipes.push(stem.to_string());
                }
            }
        }

        Ok(recipes)
    }

    pub fn delete_recipe(&self, name: &str) -> Result<()> {
        let path = self.get_recipe_path(name);
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}

fn sanitize_filename(name: &str) -> String {
    name.replace(|c: char| !c.is_ascii_alphanumeric(), "_")
        .trim_matches('_')
        .to_string()
}
