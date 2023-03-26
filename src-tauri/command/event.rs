use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transformer {
    script: EventScript, // this is also events, only tsnode
    value: Vec<EventValue>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    name: String,
    description: String,
    rule: Vec<String>, // can subscribe snowtrail event
    transformer: Transformer,
}

#[tauri::command]
pub fn create_subscribe(subscribe: Subscribe) {
    println!("{:?}", subscribe);
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventStatus {
    Process,
    Success,
    Error,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EventRuntime {
    Shell,
    Tsnode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventScript {
    runtime: EventRuntime,
    script: String, // createfile
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventValue {
    name: String,
    value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    status: EventStatus,
    script: EventScript, // 場合によっては配列に
    value: Vec<EventValue>, // like Note { name, dscription, project, save path }
    tags: Vec<String>,
}
#[tauri::command]
pub fn publish_event(event: Event) {
    println!("{:?}", event);
}


// snowtrail event
// subscribe
// transform
// publish event
// subscribe ...
