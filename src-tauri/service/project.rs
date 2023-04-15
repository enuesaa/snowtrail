use std::path::PathBuf;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::repository::rocks::RocksRepository;

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: String,
    workdir: String,
}
impl Project {
    pub fn new(name: String, workdir: String) -> Project {
        Project { name, workdir }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_workdir(&self) -> PathBuf {
        PathBuf::from(self.workdir.clone())
    }
}

pub struct ProjectService {
    rocks: RocksRepository,
}
impl ProjectService {
    pub fn new(rocks: RocksRepository) -> ProjectService {
        ProjectService { rocks }
    }

    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    pub fn list(&self) -> Vec<Project> {
        let kvs = self.rocks().list("project", "", 100);
        let mut list: Vec<Project> = vec![];
        for kv in kvs {
            list.push(serde_json::from_str(&kv.value).unwrap());
        };
        list
    }

    pub fn get(&self, id: &str) -> Project {
        let res = self.rocks().get("project", id);
        serde_json::from_str(&res.value).unwrap()
    }

    pub fn create(&self, project: Project) -> String {
        let id = Uuid::new_v4().to_string();
        self.rocks().put("project", &id, &serde_json::to_string(&project).unwrap());
        id
    }

    pub fn delete(&self, id: &str) {
        self.rocks().delete("project", id);
    }
}