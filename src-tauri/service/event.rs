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
    id: Option<String>,
    name: String,
    kvs: Vec<EventKv>, // like Note { name, dscription, project, save path }
    tags: Vec<String>,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { id: None, name: name.to_string(), kvs: vec![], tags: vec![] }
    }

    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }
    
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    
    pub fn get_kvs(&self) -> Vec<EventKv> {
        self.kvs.clone()
    }

    pub fn get_tags(&self) -> Vec<String> {
        self.tags.clone()
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
