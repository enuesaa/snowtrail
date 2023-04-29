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

pub struct SubscribeService {
    rocks: RocksRepository,
}
impl SubscribeService {
    pub fn new(rocks: RocksRepository) -> Self {
        SubscribeService { rocks }
    }

    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    pub fn list(&self) -> Vec<Subscribe> {
        let kvs = self.rocks().list("subscribe", "", 100);
        let mut list: Vec<Subscribe> = vec![];
        for kv in kvs {
            let mut subscribe: Subscribe = serde_json::from_str(&kv.value).unwrap();
            subscribe.id = Some(kv.key);
            list.push(subscribe);
        };
        list
    }

    pub fn get(&self, id: &str) -> Subscribe {
        let res = self.rocks().get("subscribe", id);
        let mut subscribe: Subscribe = serde_json::from_str(&res.value).unwrap();
        subscribe.id = Some(res.key);
        subscribe
    }

    pub fn create(&self, subscribe: Subscribe) -> String {
        let id = Uuid::new_v4().to_string();
        self.rocks().put("subscribe", &id, &serde_json::to_string(&subscribe).unwrap());
        id
    }

    pub fn delete(&self, id: &str) {
        self.rocks().delete("subscribe", id);
    }
}
