use crate::cli::SyncAction;
use crate::git_sync;
use anyhow::Result;

pub fn handle_sync(action: SyncAction) -> Result<()> {
    match action {
        SyncAction::Init { remote_url } => {
            git_sync::GitSync::init(&remote_url)?;
            println!("Repository initialized with remote: {}", remote_url);
            Ok(())
        }
        SyncAction::Pull => {
            git_sync::GitSync::pull()?;
            Ok(())
        }
        SyncAction::Push { message } => {
            let commit_message = message.unwrap_or_else(|| "Update from dina-cli".to_string());
            git_sync::GitSync::push(&commit_message)
        }
        SyncAction::Status => {
            git_sync::GitSync::status()?;
            Ok(())
        }
    }
}
