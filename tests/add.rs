#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::test_utils;

    #[test]
    fn test_handle_add_recipe() {
        let (_temp_dir, _db_path) = test_utils::setup_test_db();

        let result = handle_add_recipe("Test Recipe".to_string(), "Test Content".to_string());

        assert!(result.is_ok());

        // Überprüfen ob Rezept gespeichert wurde
        let recipes = storage::Storage::list_recipes().unwrap();
        assert!(recipes.iter().any(|r| r.name == "Test Recipe"));
    }

    #[test]
    fn test_handle_add_article() {
        let (_temp_dir, _db_path) = test_utils::setup_test_db();

        let result = handle_add_article(
            "Test Article".to_string(),
            5,
            Some("kg".to_string()),
            Some("Grains".to_string()),
            Some(2),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_add_meal_valid_day() {
        let (_temp_dir, _db_path) = test_utils::setup_test_db();

        let result = handle_add_meal("monday".to_string(), "Test Recipe".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_add_meal_invalid_day() {
        let result = handle_add_meal("invalidday".to_string(), "Test Recipe".to_string());

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Ungültiger Wochentag")
        );
    }
}
