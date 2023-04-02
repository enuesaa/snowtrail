pub mod command;
pub mod service;
pub mod repository;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::{Manager, Builder, Wry};
use repository::runcommand::Runcommand;

fn main() {
    initialize();

    create_app()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize() {
    Runcommand::initialize();
}

fn create_app() -> Builder<Wry> {
    let app = Builder::default();
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
}