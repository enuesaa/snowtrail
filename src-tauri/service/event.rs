use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Event {
    title: String,
    marketing: bool,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { title: name.to_string(), marketing: false }
    }
}
