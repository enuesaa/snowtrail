use crate::services::git::{get_git_histories, GitHistories};
use crate::repository::command::RunCommand;

#[tauri::command]
pub fn git_histories() -> GitHistories {
    get_git_histories(RunCommand::new())
}
