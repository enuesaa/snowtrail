use crate::service::dataround::Dataround;

#[tauri::command]
pub fn up_data() -> String {
    Dataround::up()
}

#[tauri::command]
pub fn status_data() -> bool {
    Dataround::is_started()
}

#[tauri::command]
pub fn down_data() {
    Dataround::down();
}
