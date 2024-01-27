use serde::{Deserialize, Serialize};
use crate::usecase::app::{AppUsecase, LogViewSchema};

// TODO mv
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogSchema {
    pub name: String,
}

#[tauri::command]
pub fn list_logs() -> Vec<LogSchema> {
    let appcase = AppUsecase::new();
    let mut res: Vec<LogSchema> = vec![];
    if let Ok(logs) = appcase.list_logs() {
        for log in logs {
            res.push(LogSchema{
                name: log,
            });
        }
    };
    res
}

#[tauri::command]
pub fn get_log(name: String) -> LogViewSchema {
    let appcase = AppUsecase::new();

    if let Ok(log) = appcase.get_log(name) {
        log
    } else {
        LogViewSchema {
            name: "".to_string(),
            content: "".to_string(),
        }
    }
}
