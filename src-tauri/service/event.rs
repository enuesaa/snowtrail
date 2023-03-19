use serde::{Serialize, Deserialize};
use crate::service::surreal::connect;
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

    pub async fn create(self) -> Result<(), Box<dyn Error>> {
        let db = connect().await?;
        db.create("event").content(self).await?;
        Ok(())
    }
}
