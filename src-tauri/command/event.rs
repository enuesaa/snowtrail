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
        ret.push(EventPublishSchema { id: event.id, name: event.name, kvs: value });
    };
    ret
}

#[tauri::command]
pub fn event_get(id: String) -> EventPublishSchema {
    println!("{:?}", id);
    let rocks = RocksRepository {};
    let event = EventService::get(rocks, &id);
    let mut data = EventPublishSchema { id: event.id, name: event.name, kvs: vec![] };
    data.kvs = event.kvs.iter().map(|v| {
        return EventPublishKvSchema { name: v.name.clone(), value: v.value.clone() }
    }).collect();
    data
}
