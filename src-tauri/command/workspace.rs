use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceSchema {
    path: String,
    auto_add_new_project: bool,
}

#[tauri::command]
pub fn get_workspace() -> WorkspaceSchema {
    WorkspaceSchema { path: "".to_string(), auto_add_new_project: true }
}

#[tauri::command]
pub fn put_workspace(data: WorkspaceSchema) {
    println!("{:?}", data);
}
