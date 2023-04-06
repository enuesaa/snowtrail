use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscribe {
    name: String
}
impl Subscribe {
    pub fn new(name: &str) -> Self {
        Subscribe { name: name.to_string() }
    }
}

pub struct SubscribeService {}
impl SubscribeService {
    pub fn list(rocks: RocksRepository) -> Vec<Subscribe> {
        todo!()
    }

    pub fn get(rocks: RocksRepository, id: &str) -> Subscribe {
        todo!()
    }

    pub fn publish(rocks: RocksRepository, subscribe: Subscribe) -> Subscribe {
        todo!()
    }

    pub fn create(rocks: RocksRepository, subscribe: Subscribe) -> String {
        todo!()
    }

    pub fn delete(rocks: RocksRepository, id: &str) {
        todo!()
    }
}
