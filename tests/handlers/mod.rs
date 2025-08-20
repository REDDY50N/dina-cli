// Helper functions for our tests

#[cfg(test)]
pub mod test_utils {
    use crate::models::{Article, Database, MealPlan, Recipe, ShoppingItem};
    use crate::storage::Storage;
    use std::path::PathBuf;
    use tempfile::tempdir;

    pub fn setup_test_db() -> (tempfile::TempDir, PathBuf) {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_data.toml");

        // -- Set temporary database path (Mock)
        std::env::set_var("TEST_DB_PATH", db_path.to_str().unwrap());

        (temp_dir, db_path)
    }

    pub fn create_test_database() -> Database {
        Database {
            recipes: vec![Recipe {
                name: "Test Recipe".to_string(),
                content: "Test content".to_string(),
            }],
            articles: vec![Article {
                name: "Test Article".to_string(),
                quantity: 5,
                unit: Some("kg".to_string()),
                category: Some("Test".to_string()),
                min_quantity: Some(2),
            }],
            meal_plans: vec![MealPlan {
                day: "monday".to_string(),
                recipe: "Test Recipe".to_string(),
            }],
            shopping_list: vec![ShoppingItem {
                id: 1,
                name: "Test Item".to_string(),
                quantity: 3,
                unit: Some("pieces".to_string()),
                store: Some("Test Store".to_string()),
                purchased: false,
            }],
        }
    }
}
