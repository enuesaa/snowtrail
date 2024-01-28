use crate::usecase::app::{AppUsecase, LogSchema};

#[tauri::command]
pub fn list_logs() -> Vec<String> {
    let appcase = AppUsecase::new();

    if let Ok(logs) = appcase.list_logs() {
        return logs;
    };
    vec![]
}

#[tauri::command]
pub fn get_log(name: String) -> LogSchema {
    let appcase = AppUsecase::new();

    if let Ok(log) = appcase.read_log(name) {
        return log;
    }
    LogSchema::from("unknown".to_string())
}
