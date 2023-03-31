use serde::{Serialize, Deserialize};
use serde_json;
use crate::repository::rocks::RocksRepository;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventKv {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    name: String,
    kvs: Vec<EventKv>, // like Note { name, dscription, project, save path }
    tags: Vec<String>,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { name: name.to_string(), kvs: vec![], tags: vec![] }
    }

    pub fn kv(mut self, name: &str, value: &str) -> Self {
        self.kvs.push(EventKv { name: name.to_string(), value: value.to_string() });
        self
    }
    
    pub fn tag(mut self, name: &str) -> Self {
        self.tags.push(name.to_string());
        self
    }
}

pub struct EventService {}
impl EventService {
    pub fn list(rocks: RocksRepository) {
        let res = rocks.list("event");
    }

    pub fn get(rocks: RocksRepository, id: &str) {
        let res = rocks.get("event", id);
    }

    pub fn create(rocks: RocksRepository, event: Event) {
        // let key = format!("event:{}", self.name.clone());
        let id = "";
        rocks.put("event", id, &serde_json::to_string(&event).unwrap())
    }

    pub fn update(rocks: RocksRepository, id: &str, event: Event) {
        rocks.put("event", id, &serde_json::to_string(&event).unwrap())
    }

    pub fn delete(rocks: RocksRepository, id: &str) {
        rocks.delete("event", id);
    }
}
