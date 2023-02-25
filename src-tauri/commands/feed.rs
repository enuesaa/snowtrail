use std::error::Error;
use rss::Channel;
use reqwest;
use serde::{Serialize};

#[derive(Serialize)]
struct FeedItem {
    title: String,
    url: String,
    description: String,
}
#[derive(Serialize)]
pub struct FeedReposne {
    title: String,
    items: Vec<FeedItem>,
}
impl FeedReposne {
    pub fn new () -> Self {
        FeedReposne { title: String::from(""), items: vec![] }
    }

    pub fn set_titlke(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn append(&mut self, title: &str, url: &str, description: &str) {
        self.items.push(FeedItem { title: String::from(title), url: String::from(url), description: String::from(description) })
    }
}

async fn feed_async(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    println!("{:#?}", channel.items()[0]);
    Ok(channel)
}

#[tauri::command]
pub fn feed(url: &str) -> FeedReposne {
    let mut res = FeedReposne::new();
    // see https://ja.stackoverflow.com/questions/88987
    if let Ok(channel) = tauri::async_runtime::block_on(feed_async(url)) {
        res.set_titlke(channel.title());

        let items = channel.items();
        items.iter().for_each(|item| {
            if let (Some(title), Some(url), Some(description)) = (item.title(), item.link(), item.description()) {
                res.append(title, url, description);
            };
        });
    };
    res
}
