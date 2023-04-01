use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Mapping {
    path: String,
    expression: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    name: String,
    description: String,
    rule: Vec<String>, // can subscribe snowtrail event
    script_id: String,
    mapping: Vec<Mapping>,
}

#[tauri::command]
pub fn create_subscribe(subscribe: Subscribe) {
    println!("{:?}", subscribe);
}
