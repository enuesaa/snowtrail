use serde::{Serialize, Deserialize};
use crate::repository::redis::Redis;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub enum EventStatus {
    Process,
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventValue {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    status: EventStatus,
    value: Vec<EventValue>, // like Note { name, dscription, project, save path }
    tags: Vec<String>,
}
impl Event {
    pub fn new(name: &str, value: Vec<EventValue>) -> Self {
        Event {
            name: name.to_string(),
            status: EventStatus::Process,
            value,
            tags: vec![]
        }
    }

    pub fn create(self) {
        let redis = Redis::new();
        let key = format!("event:{}", self.name.clone());
        let _ = redis.create(&key, &serde_json::to_string(&self).unwrap());
    }

    pub fn list() -> Vec<String> {
        let redis = Redis::new();
        let keys = redis.list("event:*");
        if let Ok(keys) = keys {
            return keys;
        };
        vec![]
    }
}
