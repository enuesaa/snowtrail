use std::process::{Command, Stdio};
use std::env;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct GitHistory {
    hash: String,
}

#[derive(Debug, Serialize)]
pub struct GitHistories {
    items: Vec<GitHistory>,
}

#[tauri::command]
pub fn git_histories() -> GitHistories {
    let mut ret = GitHistories { items: Vec::new() };
    let output = Command::new("git")
        .args(["log", "--pretty=format:%H", "-n", "5"])
        .current_dir(env::current_dir().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().split("\n").for_each(|v| {
        ret.items.push(GitHistory { hash: v.to_string() })
    });
    ret
}

#[test]
fn test_git_histories() {
    let histories = git_histories();
    assert_eq!(histories.items.len(), 5);
    assert_ne!(histories.items[0].hash, "".to_string()); // Is this meaningful ?
}
