pub mod command;
pub mod service;
pub mod repository;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::Manager;

fn main() {
    let app = tauri::Builder::default();
    let app = command::inject_commands(app);

    app
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
