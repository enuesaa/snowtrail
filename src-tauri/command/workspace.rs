use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    dir: String,
    auto_add_new_project: bool,
}

#[tauri::command]
pub fn get_workspace() -> Workspace {
    Workspace { dir: "".to_string(), auto_add_new_project: true }
}

#[tauri::command]
pub fn set_workspace(workspace: Workspace) {
    println!("{:?}", workspace);
}
