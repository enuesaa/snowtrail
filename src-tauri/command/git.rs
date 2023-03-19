use crate::service::git::{get_git_histories, GitHistories};
use crate::repository::command::Runcommand;

#[tauri::command]
pub fn gitHistories() -> GitHistories {
    get_git_histories(Runcommand::new())
}
