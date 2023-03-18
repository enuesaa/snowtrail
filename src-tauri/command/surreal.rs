use serde::Serialize;
use crate::service::surreal::{updb, downdb};

#[derive(Serialize)]
pub struct StartSurrealRepsonse {
    status: String,
}
#[tauri::command]
pub fn start_surreal() -> StartSurrealRepsonse {
    let container_id = updb();
    StartSurrealRepsonse { status: container_id }
}

#[tauri::command]
pub fn end_surreal() {
    downdb();
}
