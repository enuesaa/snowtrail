use serde::{Serialize, Deserialize};
use crate::service::script::Script;
use crate::usecase::app::AppUsecase;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    name: String,
    commands: Vec<String>,
    project_name: String,
}

#[tauri::command]
pub fn script_list(project_name: String) -> Vec<ScriptSchema> {
    let scripts = AppUsecase::new().list_scripts_in_project(project_name);
    scripts.iter().map(|s| {
        ScriptSchema {
            name: s.get_name(),
            commands: s.get_commands(),
            project_name: s.get_project_name(),
        }
    }).collect()
}

#[tauri::command]
pub fn script_get(name: String) -> ScriptSchema {
    let script = AppUsecase::new().get_script(&name);
    ScriptSchema {
        name: script.get_name(),
        commands: script.get_commands(),
        project_name: script.get_project_name(),
    }
}

#[tauri::command]
pub fn script_create(data: ScriptSchema) {
    let script = Script::new(data.name, data.commands, data.project_name);
    AppUsecase::new().create_script(script);
}

#[tauri::command]
pub fn script_delete(name: String) {
    AppUsecase::new().delete_script(&name);
}

#[tauri::command]
pub fn script_run(name: String) {
    AppUsecase::new().run_script(&name);
}
