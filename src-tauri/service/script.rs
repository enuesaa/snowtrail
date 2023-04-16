use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;

#[derive(Serialize, Deserialize)]
pub struct Script {
    name: Option<String>,
    commands: Option<Vec<String>>,
    project_name: Option<String>,
}
impl Script {
    pub fn new(name: String, commands: Vec<String>, project_name: String) -> Self {
        Script { name: Some(name), commands: Some(commands), project_name: Some(project_name) }
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or("".to_string())
    }
    
    pub fn get_commands(&self) -> Vec<String> {
        self.commands.clone().unwrap_or(vec![])
    }

    pub fn get_project_name(&self) -> String {
        self.project_name.clone().unwrap_or("".to_string())
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
        self.rocks().put("script", &script.get_name(), &serde_json::to_string(&script).unwrap());
    }

    pub fn delete(&self, name: &str) {
        self.rocks().delete("script", name);
    }
}
