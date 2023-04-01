use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptRequestRuntime {
    Shell,
    Tsnode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptRequest {
    runtime: ScriptRequestRuntime,
    script: String, // createfile
    workdir: Option<String>,
}

#[tauri::command]
pub fn create_script(req: ScriptRequest) {
    println!("{:?}", req);
}
