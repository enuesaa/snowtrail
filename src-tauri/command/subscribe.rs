use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::event::{Event, EventService, Subscribe};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeSchema {
    name: String,
    matched: String, // can subscribe event like 'snowtrail:log:{name}#publish' or 'app:event:{name}#publish'
    script_name: String,
}

#[tauri::command]
pub fn subscribe_list() -> Vec<SubscribeSchema> {
    let subscribes = EventService::list_subscribes(RocksRepository {});
    let mut ret = vec![];
    for subscribe in subscribes {
        ret.push(SubscribeSchema { name: subscribe.name, matched: "".to_string(), script_name: "".to_string() });
    };
    ret
}

#[tauri::command]
pub fn subscribe_get(id: String) -> SubscribeSchema {
    let subscribe = EventService::get_subscribe(RocksRepository {}, &id);
    SubscribeSchema { name: subscribe.name, matched: "".to_string(), script_name: "".to_string() }
}

#[tauri::command]
pub fn subscribe_create(data: SubscribeSchema) {
    let subscribe: Subscribe = Subscribe::new(&data.name);
    EventService::create_subscribe(RocksRepository {}, subscribe);
}

#[tauri::command]
pub fn subscribe_delete(id: String) {
    EventService::delete_subscribe(RocksRepository {}, &id);
}
