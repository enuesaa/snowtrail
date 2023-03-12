use serde::{Serialize, Deserialize};
use crate::services::surreal::connect;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct Person {
    title: String,
    marketing: bool,
}
impl Person {
    pub fn new(name: &str) -> Self {
        Person { title: name.to_string(), marketing: false }
    }
}

pub async fn add_person(person: Person) -> Result<(), Box<dyn Error>> {
    let db = connect().await?;
    db.create("person").content(person).await?;
    Ok(())
}
