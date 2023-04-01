use serde::{Serialize, Deserialize};
use crate::repository::runcommand::Runcommand;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptRequest {
    command: String, // createfile
    workdir: Option<String>,
}

#[tauri::command]
pub fn create_script(req: ScriptRequest) {
    println!("{:?}", req);
}

#[tauri::command]
pub fn run(req: String) -> String {
    println!("{:?}", req);
    let mut commands: Vec<&str> = req.split(" ").collect();
    commands.rotate_left(1);
    let command = commands.pop().unwrap();
    println!("{:?}", command);
    println!("{:?}", commands);

    let runcommand = Runcommand::new();
    if let Ok(stdout) = runcommand.program(command).args(commands).exec() {
        return stdout;
    };
    "".to_string()
}