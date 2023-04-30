use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::crud::Crud;
use crate::service::withid::WithId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Binding {
    pub id: Option<String>,
    pub list: Option<Vec<String>>,
}
impl Binding {
    pub fn new(name: &str) -> Self {
        Binding { id: None, list: Some(vec![]) }
    }

    pub fn add(&mut self, name: String) {
        let mut names = self.list.clone().unwrap_or(vec![]);
        names.push(name);
        self.list = Some(names);
    }

    pub fn remove(&mut self, name: String) {
        let mut names: Vec<String> = self.list.clone().unwrap_or(vec![]);
        names = names.iter().filter(|&s| s != &name).cloned().collect();
        self.list = Some(names);
    }

    pub fn list(&self) -> Vec<String> {
        self.list.clone().unwrap_or(vec![])
    }
}
impl WithId for Binding {
    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

pub struct BindingService {
    rocks: RocksRepository,
}
impl BindingService {
    pub fn new(rocks: RocksRepository) -> Self {
        BindingService { rocks }
    }

    pub fn add(&self, id: &str, name: String) {
        let mut binding = self.get(id);
        binding.add(name);
        let value = &serde_json::to_string(&binding).unwrap();
        self.rocks().put(self.cfname(), &id, &value);
    }
    
    pub fn remove(&self, id: &str, name: String) {
        let mut binding = self.get(id);
        binding.remove(name);
        let value = &serde_json::to_string(&binding).unwrap();
        self.rocks().put(self.cfname(), &id, &value);
    }
}

impl Crud<Binding> for BindingService {
    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    fn cfname(&self) -> &str {
        "binding"
    }
}
