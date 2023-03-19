use crate::service::git::GitHistories;
use crate::repository::command::Runcommand;

#[tauri::command]
#[allow(non_snake_case)]
pub fn gitHistories() -> GitHistories {
    GitHistories::fetch(Runcommand::new())
}
