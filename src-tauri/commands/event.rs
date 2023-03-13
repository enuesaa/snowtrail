use crate::services::event::{add_event as add_event_to_db, Event};

#[tauri::command]
pub async fn add_event() {
    let event = Event::new("aaa");
    let _ = add_event_to_db(event).await;
}