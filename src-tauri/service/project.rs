use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::crud::Crud;
use crate::service::withid::WithId;

#[derive(Serialize, Deserialize)]
pub struct Project {
    id: Option<String>,
    name: Option<String>,
    workdir: Option<String>,
}
impl Project {
    pub fn new(name: String, workdir: String) -> Project {
        Project { id: None, name: Some(name), workdir: Some(workdir) }
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or("".to_string())
    }

    pub fn get_workdir(&self) -> PathBuf {
        PathBuf::from(self.workdir.clone().unwrap_or("/".to_string()))
    }
}
impl WithId for Project {
    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

pub struct ProjectService {
    rocks: RocksRepository,
}
impl ProjectService {
    pub fn new(rocks: RocksRepository) -> Self {
        ProjectService { rocks }
    }
}

impl Crud<Project> for ProjectService {
    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    fn cfname(&self) -> &str {
        "project"
    }
}
