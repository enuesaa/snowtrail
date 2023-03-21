use crate::service::dataround::Dataround;
use crate::repository::command::Runcommand;

#[tauri::command]
#[allow(non_snake_case)]
pub fn upData() -> String {
    Dataround::up(Runcommand::new())
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn downData() {
    Dataround::down(Runcommand::new());
}