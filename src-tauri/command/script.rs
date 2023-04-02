use serde::{Serialize, Deserialize};
use crate::repository::runcommand::Runcommand;
use dirs::home_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    command: String, // createfile
    workdir: Option<String>,
}

#[tauri::command]
pub fn create_script(req: ScriptSchema) {
    println!("{:?}", req);
}

#[tauri::command]
pub fn run(run: String) -> String {
    println!("{:?}", run);
    let mut commands: Vec<&str> = run.split(" ").collect();
    commands.rotate_left(1);
    let command = commands.pop().unwrap();
    println!("{:?}", command);
    println!("{:?}", commands);

    let runcommand = Runcommand::new();
    let homedir = home_dir().unwrap();
    if let Ok(stdout) = runcommand.program(command).args(commands).dir(homedir).exec() {
        return stdout;
    };
    "".to_string()
}