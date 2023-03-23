use serde::{Serialize, Deserialize};
use crate::repository::redis::Redis;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct EventMeta {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    name: String,
    meta: Vec<EventMeta>,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { name: name.to_string(), meta: vec![] }
    }

    pub fn create(self) {
        let redis = Redis::new();
        let key = self.name.clone();
        let _ = redis.create(&key, &serde_json::to_string(&self).unwrap());
    }

    pub fn list() -> Vec<String> {
        let redis = Redis::new();
        let keys = redis.list("*");
        if let Ok(keys) = keys {
            return keys;
        };
        vec![]
    }
}
