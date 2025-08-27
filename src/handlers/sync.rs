use crate::cli::SyncAction;
use crate::git_sync;
use anyhow::Result;

pub fn handle_sync(action: SyncAction) -> Result<()> {
    match action {
        // -- Init database repo with remote git url
        SyncAction::Init { remote_url } => {
            git_sync::GitSync::init(&remote_url)?;
            println!("Repository initialized with remote: {}", remote_url);
            Ok(())
        }
        // -- Pull database from remote git
        SyncAction::Pull => {
            git_sync::GitSync::pull()?;
            Ok(())
        }

        // -- Push database to remote git
        SyncAction::Push { message } => {
            let commit_message = message.unwrap_or_else(|| "Update from dina-cli".to_string());
            git_sync::GitSync::push(&commit_message)
        }

        // -- Check status
        SyncAction::Status => {
            git_sync::GitSync::status()?;
            Ok(())
        }
    }
}
