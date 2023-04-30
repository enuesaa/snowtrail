use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::withid::WithId;
use crate::service::crud::Crud;
use crate::service::event::Event;

// todo re-define this struct.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscribe {
    id: Option<String>,
    name: String
}
impl Subscribe {
    pub fn new(name: &str) -> Self {
        Subscribe { id: None, name: name.to_string() }
    }

    pub fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
impl WithId for Subscribe {
    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

pub struct SubscribeService {
    rocks: RocksRepository,
}
impl SubscribeService {
    pub fn new(rocks: RocksRepository) -> Self {
        SubscribeService { rocks }
    }

    pub fn list_triggered(&self, event: Event) -> Vec<Subscribe> {
        if event.get_name() == "snowtrail:command:run" {
            vec![Subscribe::new("aa")]
        } else {
            vec![]
        }
    }
}

impl Crud<Subscribe> for SubscribeService {
    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    fn cfname(&self) -> &str {
        "subscribe"
    }
}
