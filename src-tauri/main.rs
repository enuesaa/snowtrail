use std::error::Error;
use rss::Channel;
use reqwest;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

async fn feed_async(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tauri::command]
fn feed(url: &str) -> String {
    // see https://ja.stackoverflow.com/questions/88987
    if let Ok(res) = tauri::async_runtime::block_on(feed_async(url)) {
        return res.description
    };
    String::from("failed")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            feed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
