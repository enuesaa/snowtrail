use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    name: String,
    path: String,
}
#[tauri::command]
pub fn list_projects() -> Vec<Project> {
    vec![]
}

#[tauri::command]
pub fn get_project(name: String) -> Project {
    Project { name: "".to_string(), path: "".to_string() }
}

#[tauri::command]
pub fn create_project(project: Project) {
    println!("{:?}", project);
}

#[tauri::command]
pub fn delete_project(name: String) {
    println!("{:?}", name);
}
