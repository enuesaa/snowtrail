use serde::{Serialize, Deserialize};
use crate::repository::runcommand::Runcommand;
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    name: String,
    commands: Vec<String>, // createfile
    project_id: String, // run command under this project
}

#[tauri::command]
pub fn create_script(req: ScriptSchema) {
    println!("{:?}", req);
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