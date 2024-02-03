use tauri::AppHandle;

use crate::usecase::app::{AppUsecase, ScriptSchema};
use crate::sysmenu;

#[tauri::command]
pub fn list_scripts() -> Vec<ScriptSchema> {
    let appcase = AppUsecase::new();

    if let Ok(config) = appcase.readconfig() {
        return config.scripts;
    };
    vec![]
}

#[tauri::command]
pub fn add_script(script: ScriptSchema, app: AppHandle) {
    let appcase = AppUsecase::new();

    if let Err(err) = appcase.add_script(script) {
        println!("Error: {:?}", err);
    };
    if let Err(err) = app.tray_handle().set_menu(sysmenu::create()) {
        println!("Error: {}", err.to_string());
    };
}

#[tauri::command]
pub fn remove_script(name: String, app: AppHandle) {
    let appcase = AppUsecase::new();

    if let Err(err) = appcase.remove_script(name) {
        println!("Error: {:?}", err);
    };
    if let Err(err) = app.tray_handle().set_menu(sysmenu::create()) {
        println!("Error: {}", err.to_string());
    };
}
