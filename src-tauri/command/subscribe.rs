use serde::{Serialize, Deserialize};
use crate::service::subscribe::Subscribe;
use crate::usecase::app::AppUsecase;

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeSchema {
    id: Option<String>,
    name: String,
    matched: String,
    script_name: String,
}

#[tauri::command]
pub fn subscribe_list() -> Vec<SubscribeSchema> {
    let subscribes = AppUsecase::new().list_subscribes();
    subscribes.iter().map(|s| {
        SubscribeSchema { id: s.get_id(), name: s.get_name(), matched: "".to_string(), script_name: "".to_string() }
    }).collect()
}

#[tauri::command]
pub fn subscribe_get(id: String) -> SubscribeSchema {
    let subscribe = AppUsecase::new().get_subscribe(&id);
    SubscribeSchema { id: subscribe.get_id(), name: subscribe.get_name(), matched: "".to_string(), script_name: "".to_string() }
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
