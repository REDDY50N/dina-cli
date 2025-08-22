use crate::storage::Storage;
use anyhow::Result;
use std::process::{Command, Stdio};

#[derive(thiserror::Error, Debug)]
pub enum GitError {
    #[error("Git command failed: {0}")]
    CommandFailed(String),
    #[error("Repository not initialized")]
    NotInitialized,
}

pub struct GitSync;

impl GitSync {
    /// Prüft ob das Repository initialisiert ist
    pub fn is_initialized() -> bool {
        let db_dir = match Storage::get_db_dir() {
            Ok(dir) => dir,
            Err(_) => return false,
        };

        db_dir.join(".git").exists()
    }

    /// Initialisiert das Git-Repository
    pub fn init(remote_url: &str) -> Result<()> {
        let db_dir = Storage::get_db_dir()?;

        // .gitignore erstellen
        let gitignore_path = db_dir.join(".gitignore");
        if !gitignore_path.exists() {
            std::fs::write(&gitignore_path, "*.tmp\n*.bak\n")?;
        }

        // Repository initialisieren
        if !Self::is_initialized() {
            Command::new("git")
                .arg("init")
                .current_dir(&db_dir)
                .status()
                .map_err(|e| GitError::CommandFailed(e.to_string()))?;
        }

        // Remote-URL setzen
        Command::new("git")
            .arg("remote")
            .arg("remove")
            .arg("origin")
            .current_dir(&db_dir)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .ok();

        Command::new("git")
            .arg("remote")
            .arg("add")
            .arg("origin")
            .arg(remote_url)
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        println!("Git repository initialized at {}", db_dir.display());
        Ok(())
    }

    /// Holt Änderungen vom Remote-Repository
    pub fn pull() -> Result<()> {
        let db_dir = Storage::get_db_dir()?;

        if !Self::is_initialized() {
            return Err(GitError::NotInitialized.into());
        }

        // Stash lokale Änderungen
        Command::new("git")
            .arg("stash")
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        // Pull durchführen
        let status = Command::new("git")
            .arg("pull")
            .arg("--ff-only")
            .arg("origin")
            .arg("main") // Repo branch: main (TODO: config)
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        if !status.success() {
            return Err(GitError::CommandFailed("Git pull failed".to_string()).into());
        }

        // Stash wieder anwenden
        Command::new("git")
            .arg("stash")
            .arg("pop")
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        println!("Database successfully pulled from remote repository");
        Ok(())
    }

    /// Sendet Änderungen an das Remote-Repository
    pub fn push(message: &str) -> Result<()> {
        let db_dir = Storage::get_db_dir()?;

        if !Self::is_initialized() {
            return Err(GitError::NotInitialized.into());
        }

        // Alle Änderungen hinzufügen
        let add_status = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        if !add_status.success() {
            return Err(GitError::CommandFailed("Git add failed".to_string()).into());
        }

        // Commit erstellen
        let commit_status = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        if !commit_status.success() {
            return Err(GitError::CommandFailed("Git commit failed".to_string()).into());
        }

        // Push durchführen
        let push_status = Command::new("git")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg("main")
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        if push_status.success() {
            println!("Database successfully pushed to remote repository");
            Ok(())
        } else {
            Err(GitError::CommandFailed("Git push failed".to_string()).into())
        }
    }

    /// Zeigt den aktuellen Synchronisationsstatus
    pub fn status() -> Result<()> {
        let db_dir = Storage::get_db_dir()?;

        if !Self::is_initialized() {
            return Err(GitError::NotInitialized.into());
        }

        println!("Git status:");
        Command::new("git")
            .arg("status")
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        println!("\nRemote repository:");
        Command::new("git")
            .arg("remote")
            .arg("-v")
            .current_dir(&db_dir)
            .status()
            .map_err(|e| GitError::CommandFailed(e.to_string()))?;

        Ok(())
    }
}
