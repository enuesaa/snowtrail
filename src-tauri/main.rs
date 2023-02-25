pub mod commands;

use commands::greet::greet;
use commands::feed::feed;

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
            let res = window.with_webview(|webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    let () = msg_send![webview.inner(), setAllowsBackForwardNavigationGestures: true];
                }
            });
            match res {
                Ok(_) => Ok(()),
                Err(_) => Ok(()),
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
