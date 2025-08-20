use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

static TEST_MODE: AtomicBool = AtomicBool::new(false);
static TEST_DB_PATH: parking_lot::Mutex<Option<String>> = parking_lot::Mutex::new(None);

pub struct TestConfig;

impl TestConfig {
    pub fn enable_test_mode() {
        TEST_MODE.store(true, Ordering::SeqCst);
    }

    pub fn is_test_mode() -> bool {
        TEST_MODE.load(Ordering::SeqCst)
    }

    pub fn set_test_db_path(path: String) {
        let mut guard = TEST_DB_PATH.lock();
        *guard = Some(path);
    }

    pub fn get_test_db_path() -> Option<String> {
        TEST_DB_PATH.lock().clone()
    }

    pub fn clear_test_config() {
        TEST_MODE.store(false, Ordering::SeqCst);
        let mut guard = TEST_DB_PATH.lock();
        *guard = None;
    }
}
