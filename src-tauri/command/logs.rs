use crate::usecase::app::{AppUsecase, LogSchema};
use serde::{Deserialize, Serialize};

// TODO mv
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogListSchema {
    pub name: String,
}

#[tauri::command]
pub fn list_logs() -> Vec<LogListSchema> {
    let appcase = AppUsecase::new();
    let mut res: Vec<LogListSchema> = vec![];
    if let Ok(logs) = appcase.list_logs() {
        for log in logs {
            res.push(LogListSchema { name: log });
        }
    };
    res
}

#[tauri::command]
pub fn get_log(name: String) -> LogSchema {
    let appcase = AppUsecase::new();

    if let Ok(log) = appcase.read_log(name) {
        log
    } else {
        LogSchema {
            name: "".to_string(),
            content: "".to_string(),
            time: "".to_string(),
        }
    }
}
