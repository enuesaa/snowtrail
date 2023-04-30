use serde::{Serialize, Deserialize};
use crate::service::script::Script;
use crate::usecase::app::AppUsecase;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    id: Option<String>,
    name: String,
    commands: Vec<String>,
    project_id: String,
}

#[tauri::command]
pub fn script_list(project_id: String) -> Vec<ScriptSchema> {
    let scripts = AppUsecase::new().list_scripts_in_project(&project_id);
    scripts.iter().map(|s| {
        ScriptSchema {
            id: s.get_id(),
            name: s.get_name(),
            commands: s.get_commands(),
            project_id: s.get_project_id(),
        }
    }).collect()
}

#[tauri::command]
pub fn script_get(id: String) -> ScriptSchema {
    let script = AppUsecase::new().get_script(&id);
    ScriptSchema {
        id: script.get_id(),
        name: script.get_name(),
        commands: script.get_commands(),
        project_id: script.get_project_id(),
    }
}

#[tauri::command]
pub fn script_create(data: ScriptSchema) {
    let script = Script::new(data.name, data.commands, data.project_id);
    AppUsecase::new().create_script(script);
}

#[tauri::command]
pub fn script_delete(id: String) {
    AppUsecase::new().delete_script(&id);
}

#[tauri::command]
pub fn script_run(id: String) {
    AppUsecase::new().run_script(&id);
}
