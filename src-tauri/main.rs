pub mod command;
pub mod service;
pub mod repository;

use command::*;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            dataround_up, dataround_status, dataround_down,
            event_publish,
            git_histories, push_git_histories_to_event,
            list_events,
            get_workspace, set_workspace,
            list_projects, get_project, create_project, delete_project,
            create_subscribe,
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
