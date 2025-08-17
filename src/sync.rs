use crate::storage::Storage;
use anyhow::{Context, Result};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use std::fs;
use std::io::Write;
use std::path::Path;

const GITHUB_API: &str = "https://api.github.com";
const REPO_PATH: &str = "REDDY50N/dina-database"; // Setze deinen GitHub-Benutzernamen und Repo-Namen
const DB_FILE_NAME: &str = "dina_data.toml";

#[derive(thiserror::Error, Debug)]
pub enum SyncError {
    #[error("GitHub authentication failed")]
    AuthError,
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("File system error: {0}")]
    FsError(String),
}

pub struct GitHubSync {
    token: Option<String>,
}

impl GitHubSync {
    pub fn new(token: Option<&str>) -> Self {
        GitHubSync {
            token: token.map(|t| t.to_string()),
        }
    }

    /// Holt die Datenbank von GitHub
    pub fn pull(&self) -> Result<()> {
        let client = Client::new();
        let db_path = Storage::get_db_path()?;

        let response = self
            .build_request(&client)
            .get(&format!(
                "{}/repos/{}/contents/{}",
                GITHUB_API, REPO_PATH, DB_FILE_NAME
            ))
            .send()?;

        match response.status() {
            StatusCode::OK => {
                let content: serde_json::Value = response.json()?;
                let content_str = content["content"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Invalid response format"))?;

                let decoded = base64::decode(content_str.replace("\n", ""))
                    .context("Failed to decode base64 content")?;

                fs::write(&db_path, &decoded).context("Failed to write database file")?;

                println!("Database successfully pulled from GitHub");
                Ok(())
            }
            StatusCode::NOT_FOUND => {
                anyhow::bail!("Database file not found on GitHub");
            }
            StatusCode::UNAUTHORIZED => Err(SyncError::AuthError.into()),
            _ => {
                let status = response.status();
                anyhow::bail!("GitHub API error: {}", status);
            }
        }
    }

    /// Sendet die Datenbank an GitHub
    pub fn push(&self, message: &str) -> Result<()> {
        let client = Client::new();
        let db_path = Storage::get_db_path()?;

        if !db_path.exists() {
            anyhow::bail!("Local database does not exist");
        }

        let content = fs::read(&db_path)?;
        let content_base64 = base64::encode(content);
        let sha = self.get_current_sha(&client)?;

        let mut body = serde_json::json!({
            "message": message,
            "content": content_base64,
            "branch": "main"
        });

        if let Some(sha) = sha {
            body["sha"] = serde_json::Value::String(sha);
        }

        let response = self
            .build_request(&client)
            .put(&format!(
                "{}/repos/{}/contents/{}",
                GITHUB_API, REPO_PATH, DB_FILE_NAME
            ))
            .json(&body)
            .send()?;

        match response.status() {
            StatusCode::OK | StatusCode::CREATED => {
                println!("Database successfully pushed to GitHub");
                Ok(())
            }
            StatusCode::UNAUTHORIZED => Err(SyncError::AuthError.into()),
            _ => {
                let status = response.status();
                anyhow::bail!("GitHub API error: {}", status);
            }
        }
    }

    /// Ermittelt die aktuelle SHA des Dateiinhalts
    fn get_current_sha(&self, client: &Client) -> Result<Option<String>> {
        let response = self
            .build_request(client)
            .get(&format!(
                "{}/repos/{}/contents/{}",
                GITHUB_API, REPO_PATH, DB_FILE_NAME
            ))
            .send()?;

        if response.status().is_success() {
            let content: serde_json::Value = response.json()?;
            Ok(content["sha"].as_str().map(|s| s.to_string()))
        } else {
            Ok(None)
        }
    }

    /// Erstellt einen authentifizierten Request
    fn build_request(&self, client: &Client) -> reqwest::blocking::RequestBuilder {
        let mut request = client.request();

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        request
    }
}
