use serde::{Serialize, Deserialize};
use crate::service::event::{Event, EventService};
use crate::repository::rocks::RocksRepository;

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
pub fn event_publish(req: EventPublishRequest) {
    let mut event = Event::new(&req.name);
    req.value.iter().for_each(|v| {
        event.kv(&v.name, &v.value);
    });

    let rocks = RocksRepository {};
    EventService::create(rocks, event);
}


#[tauri::command]
pub fn event_list() -> Vec<EventPublishRequest> {
    let rocks = RocksRepository {};
    let events = EventService::list(rocks);
    let mut ret = vec![];
    for event in events {
        let value: Vec<EventPublishValue> = event.kvs.iter().map(|v| {
            return EventPublishValue { name: v.name.clone(), value: v.value.clone() }
        }).collect();
        ret.push(EventPublishRequest { name: event.name, value });
    };
    ret
}
