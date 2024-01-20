use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    id: String,
    name: String,
}

#[tauri::command]
pub fn list_scripts() -> Vec<ScriptSchema> {
    let mut list = vec![];
    list.push(ScriptSchema{
        id: "aa".to_string(),
        name: "sample script".to_string(),
    });
    list.push(ScriptSchema{
        id: "bb".to_string(),
        name: "another script".to_string(),
    });
    list
}
