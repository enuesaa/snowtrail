use std::error::Error;
use rss::Channel;
use reqwest;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[allow(dead_code)]
async fn example_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://www.yahoo.com/news/rss")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
