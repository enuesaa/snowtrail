use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    name: String,
}

#[tauri::command]
pub fn list_scripts() -> Vec<ScriptSchema> {
    let mut list = vec![];
    list.push(ScriptSchema{
        name: "sample script".to_string(),
    });
    list.push(ScriptSchema{
        name: "another script".to_string(),
    });
    list
}
