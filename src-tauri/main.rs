pub mod command;
pub mod service;
pub mod repository;
pub mod usecase;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::{Manager, Builder, Wry, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use repository::runcommand::RuncommandRepository;
use command::script;

use crate::usecase::app::AppUsecase;

fn main() {
    RuncommandRepository::initialize();

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

fn create_app() -> Builder<Wry> {
    let app = Builder::default();
    let app = app.invoke_handler(tauri::generate_handler![
        script::list_scripts,
    ]);

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
                        let appcase = AppUsecase::new();
                        let runresult = appcase.run_script();
                        let item_handle = app.tray_handle().get_item(&id);
                        item_handle.set_title(runresult).unwrap();
                    }
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
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
