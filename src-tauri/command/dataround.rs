use crate::service::dataround::Dataround;

#[tauri::command]
pub fn dataround_up() -> String {
    Dataround::up()
}

#[tauri::command]
pub fn dataround_status() -> bool {
    Dataround::is_started()
}

#[tauri::command]
pub fn dataround_down() {
    Dataround::down();
}
