use serde::{Serialize, Deserialize};
use crate::repository::runcommand::RuncommandRepository;
use crate::repository::rocks::RocksRepository;
use crate::service::script::{ScriptService, Script};
use crate::service::runner::ScriptRunnerService;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    name: String,
    commands: Vec<String>, // createfile
    project_name: String, // run command under this project
}

#[tauri::command]
pub fn script_list(project_name: String) -> Vec<ScriptSchema> {
    let script_srv = ScriptService::new(RocksRepository {});
    let scripts = script_srv.list_in_project(project_name);
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
    let script_srv = ScriptService::new(RocksRepository {});
    let script = script_srv.get(&name);
    ScriptSchema {
        name: script.get_name(),
        commands: script.get_commands(),
        project_name: script.get_project_name(),
    }
}

#[tauri::command]
pub fn script_create(data: ScriptSchema) {
    let script_srv = ScriptService::new(RocksRepository {});
    script_srv.create(Script::new(data.name, data.commands, data.project_name));
}

#[tauri::command]
pub fn script_delete(name: String) {
    let script_srv: ScriptService = ScriptService::new(RocksRepository {});
    script_srv.delete(&name);
}

#[tauri::command]
pub fn script_run(name: String) {
    let script_srv = ScriptService::new(RocksRepository {});
    let script = script_srv.get(&name);
    println!("{:?}", script);
    let runner = ScriptRunnerService::new(RuncommandRepository::new());
    runner.run(script);
}
