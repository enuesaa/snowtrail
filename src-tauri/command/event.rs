use crate::service::event::Event;

#[tauri::command]
#[allow(non_snake_case)]
pub fn putEvent() {
    let event = Event::new("bb");
    event.create();
}
