use crate::usecase::app::{AppUsecase, ScriptSchema};

#[tauri::command]
pub fn list_scripts() -> Vec<ScriptSchema> {
    let appcase = AppUsecase::new();
    if let Ok(config) = appcase.readconfig() {
        return config.scripts;
    };
    vec![]
}

#[tauri::command]
pub fn add_script(script: ScriptSchema) {
    let appcase = AppUsecase::new();
    if let Err(err) = appcase.add_script(script) {
        println!("Error: {:?}", err);
    };
}

#[tauri::command]
pub fn remove_script(name: String) {
    let appcase = AppUsecase::new();
    if let Err(err) = appcase.remove_script(name) {
        println!("Error: {:?}", err);
    };
}
