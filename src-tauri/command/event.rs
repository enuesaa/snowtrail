#[derive(Debug, Serialize, Deserialize)]
pub enum EventStatus {
    Process,
    Success,
    Error,
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
    // script: EventScript, // 場合によっては配列に
    value: Vec<EventValue>, // like Note { name, dscription, project, save path }
    tags: Vec<String>,
}
#[tauri::command]
pub fn publish_event(event: Event) {
    println!("{:?}", event);
}
