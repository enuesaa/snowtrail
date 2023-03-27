use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptRuntime {
    Shell,
    Tsnode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    runtime: ScriptRuntime,
    script: String, // createfile
}

#[tauri::command]
pub fn create_script(script: Script) {
    println!("{:?}", script);
}
