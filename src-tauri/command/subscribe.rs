use serde::{Serialize, Deserialize};
use crate::service::subscribe::Subscribe;
use crate::usecase::app::AppUsecase;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeSchema {
    name: String,
    matched: String, // can subscribe event like 'snowtrail:log:{name}#publish' or 'app:event:{name}#publish'
    script_name: String,
}

#[tauri::command]
pub fn subscribe_list() -> Vec<SubscribeSchema> {
    let subscribes = AppUsecase::new().list_subscribes();
    subscribes.iter().map(|s| {
        SubscribeSchema { name: s.name.clone(), matched: "".to_string(), script_name: "".to_string() }
    }).collect()
}

#[tauri::command]
pub fn subscribe_get(id: String) -> SubscribeSchema {
    let subscribe = AppUsecase::new().get_subscribe(&id);
    SubscribeSchema { name: subscribe.name, matched: "".to_string(), script_name: "".to_string() }
}

#[tauri::command]
pub fn subscribe_create(data: SubscribeSchema) {
    let subscribe: Subscribe = Subscribe::new(&data.name);
    AppUsecase::new().create_subscribe(subscribe);
}

#[tauri::command]
pub fn subscribe_delete(id: String) {
    AppUsecase::new().delete_subscribe(&id);
}
