use crate::service::event::Event;

#[tauri::command]
pub fn put_event() {
    let event = Event::new("bb");
    event.create();
}

#[tauri::command]
pub fn list_events() -> Vec<String> {
    Event::list()
}
