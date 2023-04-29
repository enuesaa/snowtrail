use serde::{Serialize, Deserialize};
use crate::service::project::Project;
use crate::usecase::app::AppUsecase;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSchema {
    name: String,
    workdir: String,
}
#[tauri::command]
pub fn project_list() -> Vec<ProjectSchema> {
    let projects = AppUsecase::new().list_projects();
    projects.iter().map(|p| {
        ProjectSchema {
            name: p.get_name(),
            workdir: p.get_workdir().to_str().unwrap().to_string(),
        }
    }).collect()
}

#[tauri::command]
pub fn project_get(name: String) -> ProjectSchema {
    let project = AppUsecase::new().get_project(&name);
    ProjectSchema {
        name: project.get_name(),
        workdir: project.get_workdir().to_str().unwrap().to_string(),
    }
}

#[tauri::command]
pub fn project_create(data: ProjectSchema) {
    AppUsecase::new().create_project(Project::new(data.name, data.workdir));
}

#[tauri::command]
pub fn project_delete(name: String) {
    AppUsecase::new().delete_project(&name);
}
