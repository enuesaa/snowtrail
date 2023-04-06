use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeMappingSchema {
    path: String,
    expression: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeSchema {
    name: String,
    description: String,
    rule: Vec<String>, // can subscribe snowtrail event
    script_id: String,
    mapping: Vec<SubscribeMappingSchema>,
}

#[tauri::command]
pub fn create_subscribe(data: SubscribeSchema) {
    println!("{:?}", data);
    todo!();
}
