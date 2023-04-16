use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeSchema {
    name: String,
    matched: String, // can subscribe event like 'snowtrail:log:{name}#publish' or 'app:event:{name}#publish'
    script_name: String,
}

#[tauri::command]
pub fn subscribe_list() {
    todo!()
}

#[tauri::command]
pub fn subscribe_get(name: String) {
    todo!()
}

#[tauri::command]
pub fn subscribe_create(data: SubscribeSchema) {
    todo!()
}

#[tauri::command]
pub fn subscribe_delete(name: String) {
    todo!()
}
