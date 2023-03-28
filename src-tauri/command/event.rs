use serde::{Serialize, Deserialize};
use crate::service::oldevnt::Event;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventPublishValue {
    name: String,
    value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventPublishRequest {
    name: String,
    value: Vec<EventPublishValue>, // like Note { name, dscription, project, save path }
}
#[tauri::command]
pub fn event_publish(event: EventPublishRequest) {
    println!("{:?}", event);
    let event = Event::new("bb");
    event.create();
}
