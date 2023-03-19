use std::error::Error;
use rss::Channel;
use reqwest;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FeedItem {
    title: String,
    url: String,
    description: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Feed {
    title: String,
    items: Vec<FeedItem>,
}
impl Feed {
    pub fn new() -> Self {
        Feed { title: String::from(""), items: vec![] }
    }

    pub async fn fecth(url: &str) -> Result<Self, Box<dyn Error>> {
        let mut feed = Feed::new();
        let content = reqwest::get(url)
            .await?
            .bytes()
            .await?;
        let channel = Channel::read_from(&content[..])?;
        feed.set_title(channel.title());
        let items = channel.items();
        items.iter().for_each(|item| {
            if let (Some(title), Some(url), Some(description)) = (item.title(), item.link(), item.description()) {
                feed.append(title, url, description);
            };
        });
        Ok(feed)
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    
    pub fn append(&mut self, title: &str, url: &str, description: &str) {
        self.items.push(FeedItem {
            title: title.to_string(),
            url: url.to_string(),
            description: description.to_string(),
        })
    }
}