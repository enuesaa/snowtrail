use serde::{Serialize, Deserialize};
use crate::service::event::{Event, EventValue};

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
    let value = event.value.iter().map(|v| {
        EventValue { name: v.name.clone(), value: v.value.clone() }
    }).collect();
    Event::new(&event.name, value)
        .create();
}
