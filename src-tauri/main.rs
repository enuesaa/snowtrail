pub mod commands;

use commands::greet::greet;
use commands::feed::feed;
use tauri_plugin_store::StoreBuilder;
use serde_json::json;
use std::env;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            feed,
        ])
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.with_webview(|webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    let () = msg_send![webview.inner(), setAllowsBackForwardNavigationGestures: true];
                }
            })?;
            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let mut filename = env::current_dir()?;
            filename.push("aa.json");
            let mut store = StoreBuilder::new(app.handle(), filename).build();
            store.insert("a".to_string(), json!("b"))?;
            store.save()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
