use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeSchema {
    name: String,
    rule: Vec<String>, // can subscribe snowtrail event
    script_name: String,
}

#[tauri::command]
pub fn create_subscribe(data: SubscribeSchema) {
    println!("{:?}", data);
    todo!();
}
