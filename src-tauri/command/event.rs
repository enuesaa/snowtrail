use serde::{Serialize, Deserialize};
use crate::service::event::Event;
use crate::usecase::app::AppUsecase;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventPublishKvSchema {
    name: String,
    value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventPublishSchema {
    id: Option<String>,
    name: String,
    kvs: Vec<EventPublishKvSchema>, // like Note { name, dscription, project, save path }
}
#[tauri::command]
pub fn event_publish(data: EventPublishSchema) -> String {
    let mut event = Event::new(&data.name);
    data.kvs.iter().for_each(|v| {
        event.kv(&v.name, &v.value);
    });
    AppUsecase::new().create_event(event)
}


#[tauri::command]
pub fn event_list() -> Vec<EventPublishSchema> {
    let events = AppUsecase::new().list_events();
    events.iter().map(|e| {
        let value: Vec<EventPublishKvSchema> = e.kvs.iter().map(|v| {
            return EventPublishKvSchema { name: v.name.clone(), value: v.value.clone() }
        }).collect();
        EventPublishSchema { id: e.id.clone(), name: e.name.clone(), kvs: value }
    }).collect()
}

#[tauri::command]
pub fn event_get(id: String) -> EventPublishSchema {
    let event = AppUsecase::new().get_event(&id);
    EventPublishSchema {
        id: event.id,
        name: event.name,
        kvs: event.kvs.iter().map(|v| {
            EventPublishKvSchema { name: v.name.clone(), value: v.value.clone() }
        }).collect(),
    }
}
