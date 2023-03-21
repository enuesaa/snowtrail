use serde::{Serialize, Deserialize};
use crate::repository::redis::Redis;

#[derive(Serialize, Deserialize)]
pub struct Event {
    title: String,
    marketing: bool,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { title: name.to_string(), marketing: false }
    }

    pub fn create(self) {
        let mut redis = Redis::new();
        let _ = redis.create(&self.title, "a");
    }
}
