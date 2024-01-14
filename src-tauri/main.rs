pub mod command;
pub mod service;
pub mod repository;
pub mod usecase;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::{Manager, Builder, Wry, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use repository::runcommand::RuncommandRepository;

fn main() {
    initialize();

    create_app()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize() {
    RuncommandRepository::initialize();
}

fn create_app() -> Builder<Wry> {
    let app = Builder::default();
    let app = command::inject_commands(app);

    // see https://zenn.dev/izuchy/scraps/b101088f10f806
    let hey = CustomMenuItem::new("hey".to_string(), "Hey");
    let menu = SystemTrayMenu::new().add_item(hey);

    app
        .system_tray(SystemTray::new().with_menu(menu))
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
