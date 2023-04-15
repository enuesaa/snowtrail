use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::repository::rocks::RocksRepository;

#[derive(Serialize, Deserialize)]
pub struct Project {
    name: Option<String>,
    workdir: Option<String>,
}
impl Project {
    pub fn new(name: String, workdir: String) -> Project {
        Project { name: Some(name), workdir: Some(workdir) }
    }
    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or("".to_string())
    }

    pub fn get_workdir(&self) -> PathBuf {
        PathBuf::from(self.workdir.clone().unwrap_or("/".to_string()))
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

    pub fn get(&self, name: &str) -> Project {
        let res = self.rocks().get("project", name);
        serde_json::from_str(&res.value).unwrap()
    }

    pub fn create(&self, project: Project) {
        self.rocks().put("project", &project.name.clone().unwrap(), &serde_json::to_string(&project).unwrap());
    }

    pub fn delete(&self, name: &str) {
        self.rocks().delete("project", name);
    }
}