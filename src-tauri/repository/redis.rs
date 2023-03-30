use std::error::Error;
use crate::repository::repository::RepositoryTrait;
use redis::{Client, Commands, JsonCommands};

pub struct Redis {}
impl Redis {
    pub fn new() -> Self {
        Redis {}
    }

    pub fn create(self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        let client = Client::open("redis://localhost:6380/")?;
        let res = client.get_connection();
        if let Err(b) = res {
            println!("{:?}", b);
        } else {
            let mut con = res.unwrap();
            let a = con.set::<String, String, String>(key.to_string(), value.to_string());
            if let Err(a) = a {
                println!("{:?}", a);
            }
        }
        Ok(())
    }

    pub fn list(self, key: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let client = Client::open("redis://localhost:6380/")?;
        let res = client.get_connection();
        if let Err(b) = res {
            println!("{:?}", b);
        } else {
            let mut con = res.unwrap();
            let a = con.keys::<String, Vec<String>>(key.to_string());
            if let Err(a) = a {
                println!("{:?}", a);
            } else {
                return Ok(a.unwrap());
            }
        }
        Ok(vec![])
    }

    pub fn json_get(self, key: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let client = Client::open("redis://localhost:6380/")?;
        let res = client.get_connection();
        if let Err(b) = res {
            println!("{:?}", b);
        } else {
            let mut con = res.unwrap();
            let a = con.json_get::<String, String, Vec<String>>(key.to_string(), ".".to_string());
            if let Err(a) = a {
                println!("{:?}", a);
            } else {
                return Ok(a.unwrap());
            }
        }
        Ok(vec![])
    }
    
    pub fn json_set(self, key: &str, value: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let client = Client::open("redis://localhost:6380/")?;
        let res = client.get_connection();
        if let Err(b) = res {
            println!("{:?}", b);
        } else {
            let mut con = res.unwrap();
            let a = con.json_set::<String, String, String, Vec<String>>(key.to_string(), ".".to_string(), &value.to_string());
            if let Err(a) = a {
                println!("{:?}", a);
            } else {
                return Ok(a.unwrap());
            }
        }
        Ok(vec![])
    }
}

impl RepositoryTrait<()> for Redis {
    fn exec(self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}