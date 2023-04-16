use serde::{Serialize, Deserialize};
use crate::repository::runcommand::Runcommand;
use dirs::home_dir;

use super::project::ProjectSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    name: String,
    commands: Vec<String>, // createfile
    project_name: String, // run command under this project
}

#[tauri::command]
pub fn script_list(project_name: String) {
    todo!()
}

#[tauri::command]
pub fn script_get(name: String) {
    todo!()
}

#[tauri::command]
pub fn script_create(data: ProjectSchema) {
    todo!()
}

#[tauri::command]
pub fn script_delete(name: String) {
    todo!()
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