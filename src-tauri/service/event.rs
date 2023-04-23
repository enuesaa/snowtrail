use serde::{Serialize, Deserialize};
use serde_json;
use crate::repository::rocks::RocksRepository;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscribe {
    name: String
}
impl Subscribe {
    pub fn new(name: &str) -> Self {
        Subscribe { name: name.to_string() }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventKv {
    pub name: String,
    pub value: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub id: Option<String>,
    pub name: String,
    pub kvs: Vec<EventKv>, // like Note { name, dscription, project, save path }
    pub tags: Vec<String>,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { id: None, name: name.to_string(), kvs: vec![], tags: vec![] }
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
        println!("{:?}", kvs);
        let mut list: Vec<Event> = vec![];
        for kv in kvs {
            let mut event: Event = serde_json::from_str(&kv.value).unwrap();
            event.id = Some(kv.key);
            list.push(event);
        };
        list
    }

    pub fn get(rocks: RocksRepository, id: &str) -> Event {
        let res = rocks.get("event", id);
        let mut event: Event = serde_json::from_str(&res.value).unwrap();
        event.id = Some(res.key);
        event
    }

    pub fn publish(rocks: RocksRepository, event: Event) -> String {
        let id = EventService::create(rocks.clone(), event.clone());
        EventService::trigger(rocks, &id);
        id
    }

    pub fn create(rocks: RocksRepository, event: Event) -> String {
        let id = Uuid::new_v4().to_string();
        rocks.put("event", &id, &serde_json::to_string(&event).unwrap());
        id
    }

    pub fn trigger(rocks: RocksRepository, id: &str) {
        let res = rocks.get("event", id);
        let mut event: Event = serde_json::from_str(&res.value).unwrap();
        let subscribes = "";
    }

    pub fn delete(rocks: RocksRepository, id: &str) {
        rocks.delete("event", id);
    }

    pub fn list_subscribes() -> Vec<Subscribe> {
        todo!()
    }

    pub fn get_subscribe(id: &str) -> Subscribe {
        todo!()
    }

    pub fn create_subscribe(subscribe: Subscribe) -> String {
        todo!()
    }

    pub fn delete_subscribe(id: &str) {
        todo!()
    }
}
