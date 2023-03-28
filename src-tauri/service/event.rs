use serde::{Serialize, Deserialize};

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
    value: Vec<EventValue>, // like Note { name, dscription, project, save path }
    tags: Vec<String>,
}
