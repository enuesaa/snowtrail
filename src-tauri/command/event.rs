use serde::{Serialize, Deserialize};
use crate::service::event::{Event, EventService};
use crate::repository::rocks::RocksRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventPublishKvSchema {
    name: String,
    value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventPublishSchema {
    name: String,
    kvs: Vec<EventPublishKvSchema>, // like Note { name, dscription, project, save path }
}
#[tauri::command]
pub fn event_publish(data: EventPublishSchema) -> String {
    let mut event = Event::new(&data.name);
    data.kvs.iter().for_each(|v| {
        event.kv(&v.name, &v.value);
    });

    let rocks = RocksRepository {};
    let id = EventService::publish(rocks, event);
    id
}


#[tauri::command]
pub fn event_list() -> Vec<EventPublishSchema> {
    let rocks = RocksRepository {};
    let events = EventService::list(rocks);
    let mut ret = vec![];
    for event in events {
        let value: Vec<EventPublishKvSchema> = event.kvs.iter().map(|v| {
            return EventPublishKvSchema { name: v.name.clone(), value: v.value.clone() }
        }).collect();
        ret.push(EventPublishSchema { name: event.name, kvs: value });
    };
    ret
}
