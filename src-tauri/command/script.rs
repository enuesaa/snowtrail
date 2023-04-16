use serde::{Serialize, Deserialize};
use crate::repository::runcommand::Runcommand;
use dirs::home_dir;
use crate::repository::rocks::RocksRepository;
use crate::service::script::{ScriptService, Script};

use super::project::ProjectSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    name: String,
    commands: Vec<String>, // createfile
    project_name: String, // run command under this project
}

#[tauri::command]
pub fn script_list(project_name: String) -> Vec<ScriptSchema> {
    let script_srv = ScriptService::new(RocksRepository {});
    let scripts = script_srv.list();
    scripts.iter().map(|s| {
        ScriptSchema {
            name: "".to_string(),
            commands: vec![],
            project_name: "".to_string(),
        }
    }).collect()
}

#[tauri::command]
pub fn script_get(name: String) -> ScriptSchema {
    let script_srv = ScriptService::new(RocksRepository {});
    let script = script_srv.get(&name);
    ScriptSchema {
        name: "".to_string(),
        commands: vec![],
        project_name: "".to_string(),
    }
}

#[tauri::command]
pub fn script_create(data: ProjectSchema) {
    let script_srv = ScriptService::new(RocksRepository {});
    script_srv.create(Script::new(""));
}

#[tauri::command]
pub fn script_delete(name: String) {
    let script_srv = ScriptService::new(RocksRepository {});
    script_srv.delete(&name);
}

#[tauri::command]
pub fn script_run(name: String) {
    todo!()
}

#[tauri::command]
pub fn run(run: String) -> String {
    let mut args: Vec<&str> = run.split(" ").collect();
    args.rotate_left(1);
    let command = args.pop().unwrap();

    let runcommand = Runcommand::new();
    let homedir = home_dir().unwrap();
    if let Ok(stdout) = runcommand.program(command).args(args).dir(homedir).exec() {
        return stdout;
    };
    "".to_string()
}