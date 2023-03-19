use crate::service::event::Event;

#[tauri::command]
#[allow(non_snake_case)]
pub async fn addEvent() {
    let event = Event::new("aaa");
    let _ = event.create().await;
}
