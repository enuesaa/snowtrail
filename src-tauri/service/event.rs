use serde::{Serialize, Deserialize};
use serde_json;
use crate::repository::rocks::RocksRepository;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventKv {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub kvs: Vec<EventKv>, // like Note { name, dscription, project, save path }
    pub tags: Vec<String>,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { name: name.to_string(), kvs: vec![], tags: vec![] }
    }

    pub fn kv(&mut self, name: &str, value: &str) {
        self.kvs.push(EventKv { name: name.to_string(), value: value.to_string() });
    }
    
    pub fn tag(&mut self, name: &str) {
        self.tags.push(name.to_string());
    }
}

pub struct EventService {}
impl EventService {
    pub fn list(rocks: RocksRepository) -> Vec<Event> {
        let kvs = rocks.list("event", "", 100);
        let mut list: Vec<Event> = vec![];
        for kv in kvs {
            list.push(serde_json::from_str(&kv.value).unwrap());
        };
        list
    }

    pub fn get(rocks: RocksRepository, id: &str) -> Event {
        let res = rocks.get("event", id);
        serde_json::from_str(&res.value).unwrap()
    }

    pub fn create(rocks: RocksRepository, event: Event) {
        let id = &Uuid::new_v4().to_string();
        rocks.put("event", id, &serde_json::to_string(&event).unwrap())
    }

    pub fn update(rocks: RocksRepository, id: &str, event: Event) {
        rocks.put("event", id, &serde_json::to_string(&event).unwrap())
    }

    pub fn delete(rocks: RocksRepository, id: &str) {
        rocks.delete("event", id);
    }
}
