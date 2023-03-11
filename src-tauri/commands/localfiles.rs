use std::env;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct LocalfileItem {
    name: String,
}
#[derive(Debug, Serialize)]
pub struct LocalfilesResponse {
    items: Vec<LocalfileItem>,
}

#[tauri::command]
pub fn localfiles() -> LocalfilesResponse {
    let mut res = LocalfilesResponse { items: Vec::new() };

    let dir = env::current_dir().unwrap().parent().unwrap().to_owned();
    for file in dir.read_dir().unwrap() {
        res.items.push(LocalfileItem {
            name: file.unwrap().path().to_str().unwrap().to_string()
        });
    };
    println!("{:?}", res);
    res
}
