use std::error::Error;
use rss::Channel;
use reqwest;

async fn feed_async(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    // println!("{:?}", channel.items()[0]);
    Ok(channel)
}

#[tauri::command]
pub fn feed(url: &str) -> Vec<String> {
    // see https://ja.stackoverflow.com/questions/88987
    if let Ok(channel) = tauri::async_runtime::block_on(feed_async(url)) {
        let items = channel.items();
        let titles: Vec<String> = items.iter().map(|item| {
            if let Some(title) = item.title() {
                return String::from(title);
            };
            return String::from("");
        }).collect();
        return titles;
    };
    vec![String::from("failed")]
}
