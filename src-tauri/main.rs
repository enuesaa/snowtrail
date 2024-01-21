pub mod command;
pub mod service;
pub mod repository;
pub mod usecase;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::{Manager, Builder, Wry, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use repository::runcommand::RuncommandRepository;
use command::scripts;

use crate::usecase::app::{AppUsecase, ConfigSchema};

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
        scripts::list_scripts,
        scripts::add_script,
        scripts::remove_script,
    ]);

    // see https://zenn.dev/izuchy/scraps/b101088f10f806
    let hey = CustomMenuItem::new("hey".to_string(), "Hey");
    let saver = CustomMenuItem::new("saver".to_string(), "Saver");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let menu = SystemTrayMenu::new()
        .add_item(hey)
        .add_item(saver)
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
                    "saver" => {
                        let appcase = AppUsecase::new();
                        let config = ConfigSchema {
                            updated: "2024-01-21T15:16:00+09:00".to_string(),
                            scripts: vec![],                            
                        };
                        let result = appcase.writeconfig(config);
                        println!("{:?}", result);

                        let result = appcase.readconfig();
                        println!("{:?}", result);
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
