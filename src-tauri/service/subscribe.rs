use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::withid::WithId;
use crate::service::crud::Crud;

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
}

impl Crud<Subscribe> for SubscribeService {
    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    fn cfname(&self) -> &str {
        "subscribe"
    }
}
