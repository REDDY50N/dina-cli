// tests/test_utils.rs
use dina_cli::config::TestConfig;
use tempfile::tempdir;

pub struct TestEnvironment {
    _temp_dir: tempfile::TempDir,
}

impl TestEnvironment {
    pub fn new() -> Self {
        TestConfig::enable_test_mode();

        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir
            .path()
            .join("test_data.toml")
            .to_str()
            .unwrap()
            .to_string();

        TestConfig::set_test_db_path(db_path);

        TestEnvironment {
            _temp_dir: temp_dir,
        }
    }
}

impl Drop for TestEnvironment {
    fn drop(&mut self) {
        TestConfig::clear_test_config();
    }
}
