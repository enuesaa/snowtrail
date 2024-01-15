pub mod command;
pub mod service;
pub mod repository;
pub mod usecase;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::{Manager, Builder, Wry, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use repository::runcommand::RuncommandRepository;

fn main() {
    initialize();

    // see https://tauri.app/v1/guides/features/system-tray/#preventing-the-app-from-closing
    create_app()
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}

fn initialize() {
    RuncommandRepository::initialize();
}

fn create_app() -> Builder<Wry> {
    let app = Builder::default();
    let app = command::inject_commands(app);

    // see https://zenn.dev/izuchy/scraps/b101088f10f806
    let hey = CustomMenuItem::new("hey".to_string(), "Hey");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let menu = SystemTrayMenu::new()
        .add_item(hey)
        .add_item(hide)
        .add_item(quit);

    app
        .system_tray(SystemTray::new().with_menu(menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "hey" => {
                        println!("hey clicked.");
                    }
                    "quit" => {
                        println!("quit clicked.");
                        std::process::exit(0);
                    }
                    "hide" => {
                        println!("hide clicked.");
                        let window = app.get_window("main").unwrap();
                        window.hide().unwrap();
                    }
                    _ => {}
                }
            }
            _ => {}
          })
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
