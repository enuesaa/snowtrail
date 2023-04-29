use serde::{Serialize, Deserialize};
use serde_json;
use crate::repository::rocks::RocksRepository;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Subscribe {
    pub id: Option<String>,
    pub name: String
}
impl Subscribe {
    pub fn new(name: &str) -> Self {
        Subscribe { id: None, name: name.to_string() }
    }
}

pub struct SubscribeService {}
impl SubscribeService {
    pub fn list(rocks: RocksRepository) -> Vec<Subscribe> {
        let kvs = rocks.list("subscribe", "", 100);
        let mut list: Vec<Subscribe> = vec![];
        for kv in kvs {
            let mut subscribe: Subscribe = serde_json::from_str(&kv.value).unwrap();
            subscribe.id = Some(kv.key);
            list.push(subscribe);
        };
        list
    }

    pub fn get(rocks: RocksRepository, id: &str) -> Subscribe {
        let res = rocks.get("subscribe", id);
        let mut subscribe: Subscribe = serde_json::from_str(&res.value).unwrap();
        subscribe.id = Some(res.key);
        subscribe
    }

    pub fn create(rocks: RocksRepository, subscribe: Subscribe) -> String {
        let id = Uuid::new_v4().to_string();
        rocks.put("subscribe", &id, &serde_json::to_string(&subscribe).unwrap());
        id
    }

    pub fn delete(rocks: RocksRepository, id: &str) {
        rocks.delete("subscribe", id);
    }
}
