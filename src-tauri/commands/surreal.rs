use std::process::{Command, Stdio};
use serde::Serialize;

#[derive(Serialize)]
pub struct StartSurrealRepsonse {
    status: String,
}
#[tauri::command]
pub fn start_surreal() -> StartSurrealRepsonse {
    let output = Command::new("docker")
        .args(["run", "-d", "--rm", "--name", "snowtrail-surreal", "-p", "8000:8000", "surrealdb/surrealdb:latest", "start"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    StartSurrealRepsonse { status: String::from_utf8(output.stdout).unwrap() }
}

#[tauri::command]
pub fn end_surreal() {
    Command::new("docker")
        .args(["stop", "snowtrail-surreal"])
        .output()
        .unwrap();
}
