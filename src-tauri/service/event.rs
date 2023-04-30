use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::crud::Crud;
use crate::service::withid::WithId;

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
impl WithId for Event {
    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

pub struct EventService {
    rocks: RocksRepository,
}
impl EventService {
    pub fn new(rocks: RocksRepository) -> Self {
        EventService { rocks }
    }
}

impl Crud<Event> for EventService {
    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }
    fn cfname(&self) -> &str {
        "event"
    }
}
