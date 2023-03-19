pub mod command;
pub mod service;
pub mod repository;

use command::feed::feed;
use command::localfiles::localfiles;
use command::surreal::{startSurreal, endSurreal};
use command::event::addEvent;
use command::git::gitHistories;
use command::greet::greet;
use std::env;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            feed,
            localfiles,
            startSurreal,
            endSurreal,
            addEvent,
            gitHistories,
            greet,
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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
