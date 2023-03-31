use crate::service::oldevnt::Event;

#[tauri::command]
pub fn list_events() -> Vec<String> {
    Event::list()
}
