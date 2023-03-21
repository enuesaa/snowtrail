use crate::service::dataround::Dataround;

#[tauri::command]
#[allow(non_snake_case)]
pub fn upData() -> String {
    Dataround::up()
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn downData() {
    Dataround::down();
}