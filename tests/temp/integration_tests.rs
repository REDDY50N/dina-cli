use assert_cmd::Command;
use lazy_static::lazy_static;
use predicates::prelude::*;
use std::sync::Mutex;
use tempfile::tempdir;

lazy_static! {
    static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

#[test]
fn test_cli_add_recipe() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp_dir = tempdir().unwrap();

    // Safe: Im Test-Kontext mit Mutex
    unsafe {
        std::env::set_var(
            "TEST_DB_PATH",
            temp_dir.path().join("test_db.toml").to_str().unwrap(),
        );
    }

    let mut cmd = Command::cargo_bin("dina-cli").unwrap();

    cmd.args(&["add", "recipe", "Test Recipe", "Test Content"])
        .assert()
        .success();
}

#[test]
fn test_cli_list_recipes() {
    let _guard = TEST_MUTEX.lock().unwrap();
    let temp_dir = tempdir().unwrap();

    unsafe {
        std::env::set_var(
            "TEST_DB_PATH",
            temp_dir.path().join("test_db.toml").to_str().unwrap(),
        );
    }

    // Erst hinzufügen, dann auflisten
    let mut cmd = Command::cargo_bin("dina-cli").unwrap();
    cmd.args(&["add", "recipe", "Test Recipe", "Test Content"])
        .assert()
        .success();

    cmd.args(&["list", "recipes"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Recipe"));
}
