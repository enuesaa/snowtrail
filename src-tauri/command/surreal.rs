use crate::service::surreal_management::SurrealManagement;
use crate::repository::command::Runcommand;

#[tauri::command]
#[allow(non_snake_case)]
pub fn startSurreal() -> String {
    SurrealManagement::up(Runcommand::new())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn endSurreal() {
    SurrealManagement::down(Runcommand::new());
}
