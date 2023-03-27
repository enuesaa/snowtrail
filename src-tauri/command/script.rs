#[derive(Debug, Serialize, Deserialize)]
pub enum ScriptRuntime {
    Shell,
    Tsnode,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
    runtime: EventRuntime,
    script: String, // createfile
}

#[tauri::command]
pub fn create_script(script: Script) {
    println!("{:?}", script);
}
