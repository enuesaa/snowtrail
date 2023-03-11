use std::process::{Command, Stdio};

#[tauri::command]
pub fn runls() -> String {
    let output = Command::new("ls")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}
