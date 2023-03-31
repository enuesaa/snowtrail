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
pub fn event_list() {
    let rocks = RocksRepository {};
    let res = EventService::list(rocks);
    println!("{:?}", res);
}
