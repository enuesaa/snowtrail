use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSchema {
    name: String,
    path: String,
}
#[tauri::command]
pub fn list_projects() -> Vec<ProjectSchema> {
    vec![]
}

#[tauri::command]
pub fn get_project(name: String) -> ProjectSchema {
    ProjectSchema { name: name, path: "".to_string() }
}

#[tauri::command]
pub fn create_project(project: ProjectSchema) {
    println!("{:?}", project);
}

#[tauri::command]
pub fn delete_project(name: String) {
    println!("{:?}", name);
}
