use serde::{Serialize, Deserialize};
use serde_json;
use crate::repository::rocks::RocksRepository;

#[derive(Serialize, Deserialize)]
struct Workspace {
    path: Option<String>,
    auto_add_new_project: bool,
}

pub struct WorkspaceService {
    rocks: RocksRepository,
    workspace: Workspace,
}
impl WorkspaceService {
    pub fn new(rocks: RocksRepository) -> WorkspaceService {
        let res = rocks.clone().get("workspace", "main");
        let workspace: Workspace = serde_json::from_str(&res.value).unwrap();
        WorkspaceService { rocks, workspace }
    }

    pub fn get_path(self) -> Option<String> {
        self.workspace.path
    }

    pub fn update_path(mut self, path: Option<String>) {
        self.workspace.path = path;
        self.save();
    }

    pub fn get_auto_add_new_project(self) -> bool {
        self.workspace.auto_add_new_project
    }

    pub fn update_auto_add_new_project(mut self, is: bool) {
        self.workspace.auto_add_new_project = is;
        self.save();
    }

    fn save(self) {
        self.rocks.put("workspace", "main", &serde_json::to_string(&self.workspace).unwrap());
    }
}
