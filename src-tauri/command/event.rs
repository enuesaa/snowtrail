use crate::service::event::Event;

#[tauri::command]
#[allow(non_snake_case)]
pub fn putEvent() {
    let mut event = Event::new("aa");
    event.create();
}
