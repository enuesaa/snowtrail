pub mod command;
pub mod repository;
pub mod usecase;
pub mod init;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use tauri::{Manager, Builder, CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent};
use command::scripts;
use std::process;

use crate::usecase::app::AppUsecase;

fn main() {
    if let Err(err) = init::init() {
        println!("Error: {}", err.to_string());
        process::exit(1);
    };

    let appcase = AppUsecase::new();
    let mut menu = SystemTrayMenu::new();
    if let Ok(config) = appcase.readconfig() {
        for script in config.scripts {
            let item = CustomMenuItem::new(script.name.clone(), script.name.clone());
            menu = menu.add_item(item);
        };
    };
    // see https://zenn.dev/izuchy/scraps/b101088f10f806
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    menu = menu.add_item(quit);

    let app = Builder::default()
        .invoke_handler(tauri::generate_handler![
            scripts::list_scripts,
            scripts::add_script,
            scripts::remove_script,
        ])
        .system_tray(SystemTray::new().with_menu(menu))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if id.as_str() == "quit" {
                    std::process::exit(0);
                };

                let appcase = AppUsecase::new();
                if let Ok(script) = appcase.get_script(id.clone()) {
                    println!("run: {:?}", script.command);
                    let runresult = appcase.run_script(script);
                    println!("result: {:?}", runresult);

                    let item_handle = app.tray_handle().get_item(&id);
                    if let Ok(_) = runresult {
                        item_handle.set_title("OK").unwrap();
                    } else {
                        item_handle.set_title("ERR").unwrap();
                    };
                    return;
                };
                println!("err: not found.");
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
