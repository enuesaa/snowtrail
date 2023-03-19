use crate::service::git::{get_git_histories, GitHistories};
use crate::repository::command::Runcommand;

#[tauri::command]
#[allow(non_snake_case)]
pub fn gitHistories() -> GitHistories {
    get_git_histories(Runcommand::new())
}
