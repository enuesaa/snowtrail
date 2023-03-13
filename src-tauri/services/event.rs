use serde::{Serialize, Deserialize};
use crate::services::surreal::connect;
use std::error::Error;

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

pub async fn add_event(event: Event) -> Result<(), Box<dyn Error>> {
    let db = connect().await?;
    db.create("event").content(event).await?;
    Ok(())
}
