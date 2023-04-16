use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;

#[derive(Serialize, Deserialize)]
pub struct Script {
    name: Option<String>,
    commands: Vec<String>,
    project_name: String,
}
impl Script {
    pub fn new(name: &str) -> Self {
        Script { name: Some(name.to_string()), commands: vec![], project_name: "".to_string() }
    }
}

pub struct ScriptService {
    rocks: RocksRepository,
}
impl ScriptService {
    pub fn new(rocks: RocksRepository) -> Self {
        ScriptService { rocks }
    }

    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    pub fn list(&self) -> Vec<Script> {
        let kvs = self.rocks().list("script", "", 100);
        let mut list: Vec<Script> = vec![];
        for kv in kvs {
            list.push(serde_json::from_str(&kv.value).unwrap());
        };
        list
    }

    pub fn get(&self, name: &str) -> Script {
        let res = self.rocks().get("script", name);
        serde_json::from_str(&res.value).unwrap()
    }

    pub fn create(&self, script: Script) {
        self.rocks().put("script", &script.name.clone().unwrap(), &serde_json::to_string(&script).unwrap());
    }

    pub fn delete(&self, name: &str) {
        self.rocks().delete("script", name);
    }
}
