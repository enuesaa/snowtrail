use serde::{Serialize, Deserialize};

use crate::usecase::app::{AppUsecase, Script};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScriptSchema {
    name: String,
}

#[tauri::command]
pub fn list_scripts() -> Vec<ScriptSchema> {
    let mut list = vec![];

    let appcase = AppUsecase::new();
    if let Ok(config) = appcase.readconfig() {
        for script in config.scripts {
            list.push(ScriptSchema { name: script.name })
        };
    };
    list
}

#[tauri::command]
pub fn add_script(script: ScriptSchema) {
    let appcase = AppUsecase::new();
    let result = appcase.add_script(Script {
        name: script.name,
        command: vec!["echo".to_string(), "aa".to_string()],
        description: "example example".to_string(),
        pid: None,
    });
    println!("{:?}", result);
}

#[tauri::command]
pub fn remove_script(name: String) {
    let appcase = AppUsecase::new();
    let result = appcase.remove_script(name);
    println!("{:?}", result);
}
