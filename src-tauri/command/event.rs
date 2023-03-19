use crate::service::event::Event;

#[tauri::command]
#[allow(non_snake_case)]
pub async fn addEvent() {
    // let _ = Event::create("aaa").await;
}
