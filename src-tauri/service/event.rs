use serde::{Serialize, Deserialize};
use crate::repository::surreal::Surreal;
use std::error::Error;
#[allow(unused_imports)]
use crate::repository::repository::RepositoryTrait;

#[derive(Serialize, Deserialize)]
pub struct Event {
    title: String,
    marketing: bool,
}
impl Event {
    pub fn new(name: &str) -> Self {
        Event { title: name.to_string(), marketing: false }
    }

    // pub async fn create(name: &str) -> Result<Self, Box<dyn Error>> {
    //     let event = Event::new(name);
    //     let res = Surreal::new().exec();
    //     if let Ok(db) = res {
    //         db.create("event").content(&event).await?;
    //         Ok(event);
    //     };
    // }
}
