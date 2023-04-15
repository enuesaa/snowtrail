use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::workspace::WorkspaceService;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceSchema {
    path: String,
    auto_add_new_project: bool,
}

#[tauri::command]
pub fn get_workspace() -> WorkspaceSchema {
    let rocks = RocksRepository {};
    let workspace_srv = WorkspaceService::new(rocks);
    WorkspaceSchema {
        path: workspace_srv.get_path().unwrap_or("".to_string()),
        auto_add_new_project: workspace_srv.get_auto_add_new_project(),
    }
}

#[tauri::command]
pub fn set_workspace(data: WorkspaceSchema) {
    let rocks = RocksRepository {};
    let mut workspace_srv = WorkspaceService::new(rocks);
    workspace_srv.update_auto_add_new_project(data.auto_add_new_project);
    workspace_srv.update_path(Some(data.path));

}
