use serde::{Serialize, Deserialize};
use crate::service::event::Event;

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
    Event::new(&event.name)
        .set_value(&event.value.first().unwrap().name, &event.value.first().unwrap().value)
        .create();
}
