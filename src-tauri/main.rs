pub mod command;
pub mod init;
pub mod repository;
pub mod usecase;

#[cfg(target_os = "macos")]
#[macro_use]
extern crate objc;
use command::logs;
use command::scripts;
use std::process;
use tauri::{Builder, Manager, SystemTray, SystemTrayEvent};

use crate::usecase::app::AppUsecase;

#[tokio::main]
async fn main() {
    if let Err(err) = init::init() {
        println!("Error: {}", err.to_string());
        process::exit(1);
    };

    let app = Builder::default()
        .invoke_handler(tauri::generate_handler![
            scripts::list_scripts,
            scripts::add_script,
            scripts::remove_script,
            logs::list_logs,
            logs::get_log,
        ])
        .system_tray(SystemTray::new().with_menu(init::create_menu()))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                if id.as_str() == "reload" {
                    if let Err(err) = app.tray_handle().set_menu(init::create_menu()) {
                        println!("Error: {}", err.to_string());
                    };
                    return;
                };

                if id.as_str() == "open" {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.show();
                    } else {
                        println!("window nt found.");
                    };
                    return;
                };

                if id.as_str() == "quit" {
                    std::process::exit(0);
                };

                let mut appcase = AppUsecase::new();
                if let Ok(script) = appcase.get_script(id.clone()) {
                    println!("run: {:?}", script.command);
                    let item_handle = app.tray_handle().get_item(&id);
                    item_handle.set_title("Running").unwrap();
                    tokio::spawn(async move {
                        let _ = appcase.run_script(script).await;
                        item_handle.set_title("OK").unwrap();
                    });
                    return;
                };
                println!("err: not found.");
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            // see https://tauri.app/v1/guides/features/system-tray/#preventing-the-app-from-closing
            tauri::WindowEvent::CloseRequested { api, .. } => {
              event.window().hide().unwrap();
              api.prevent_close();
            }
            _ => {}
        })
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            window.with_webview(|webview| {
                #[cfg(target_os = "macos")]
                unsafe {
                    let () =
                        msg_send![webview.inner(), setAllowsBackForwardNavigationGestures: true];
                }
            })?;
            Ok(())
        });

    app.build(tauri::generate_context!())
        .expect("failed to start app")
        .run(|_app_handle, event| match event {
            // see https://tauri.app/v1/guides/features/system-tray/#preventing-the-app-from-closing
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
