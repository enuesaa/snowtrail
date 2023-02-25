pub mod commands;

use commands::greet::greet;
use commands::feed::feed;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            feed,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
