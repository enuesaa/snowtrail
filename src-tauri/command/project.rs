use serde::{Serialize, Deserialize};
use crate::repository::rocks::RocksRepository;
use crate::service::project::{ProjectService, Project};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectSchema {
    name: String,
    workdir: String,
}
#[tauri::command]
pub fn project_list() -> Vec<ProjectSchema> {
    let project_srv = ProjectService::new(RocksRepository {});
    let projects = project_srv.list();
    projects.iter().map(|p| {
        ProjectSchema {
            name: p.get_name(),
            workdir: p.get_workdir().to_str().unwrap().to_string(),
        }
    }).collect()
}

#[tauri::command]
pub fn project_get(name: String) -> ProjectSchema {
    let project_srv = ProjectService::new(RocksRepository {});
    let project = project_srv.get(&name);
    ProjectSchema {
        name: project.get_name(),
        workdir: project.get_workdir().to_str().unwrap().to_string(),
    }
}

#[tauri::command]
pub fn project_create(data: ProjectSchema) {
    let project_srv = ProjectService::new(RocksRepository {});
    project_srv.create(Project::new(data.name, data.workdir));
}

#[tauri::command]
pub fn project_delete(name: String) {
    let project_srv = ProjectService::new(RocksRepository {});
    project_srv.delete(&name);
}
