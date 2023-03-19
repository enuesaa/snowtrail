use serde::Serialize;
use crate::service::surreal::{updb, downdb};

#[derive(Serialize)]
pub struct StartSurrealRepsonse {
    status: String,
}
#[tauri::command]
#[allow(non_snake_case)]
pub fn startSurreal() -> StartSurrealRepsonse {
    let container_id = updb();
    StartSurrealRepsonse { status: container_id }
}

#[tauri::command]
#[allow(non_snake_case)]
pub fn endSurreal() {
    downdb();
}
