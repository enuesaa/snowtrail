use crate::service::feed::Feed;

#[tauri::command]
pub fn feed(url: &str) -> Feed {
    // see https://ja.stackoverflow.com/questions/88987
    if let Ok(feed) = tauri::async_runtime::block_on(Feed::fecth(url)) {
        feed
    } else {
        Feed::new()
    }
}
