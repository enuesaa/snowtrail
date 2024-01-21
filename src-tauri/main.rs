pub mod command;
pub mod repository;
pub mod usecase;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::{Manager, Builder, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use repository::runcommand::RuncommandRepository;
use command::scripts;

use crate::usecase::app::{AppUsecase, ConfigSchema};

fn main() {
    RuncommandRepository::initialize();
    let appcase = AppUsecase::new();
    if let Ok(is) = appcase.is_registry_exist() {
        if !is {
            let _ = appcase.create_registry();
            let config = ConfigSchema {
                updated: "2024-01-21T15:16:00+09:00".to_string(),
                scripts: vec![],                            
            };
            let _ = appcase.writeconfig(config);
        }
    }

    // see https://zenn.dev/izuchy/scraps/b101088f10f806
    let hey = CustomMenuItem::new("hey".to_string(), "Hey");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let menu = SystemTrayMenu::new()
        .add_item(hey)
        .add_item(quit);

    let app = Builder::default()
        .invoke_handler(tauri::generate_handler![
            scripts::list_scripts,
            scripts::add_script,
            scripts::remove_script,
        ])
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
        });

    app
        .build(tauri::generate_context!())
        .expect("failed to start app")
        .run(|_app_handle, event| match event {
            // see https://tauri.app/v1/guides/features/system-tray/#preventing-the-app-from-closing
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
