pub mod command;
pub mod init;
pub mod sysmenu;
pub mod repository;
pub mod usecase;

use command::logs;
use command::scripts;
use std::process;
use tauri::{Builder, SystemTray, SystemTrayEvent};

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
        .system_tray(SystemTray::new().with_menu(sysmenu::create()))
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => sysmenu::handleclick(app, id.as_str()),
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
              event.window().hide().unwrap();
              api.prevent_close();
            },
            _ => {},
        });

    app.build(tauri::generate_context!())
        .expect("failed to start app")
        .run(|_, event| match event {
            // see https://tauri.app/v1/guides/features/system-tray/#preventing-the-app-from-closing
            tauri::RunEvent::ExitRequested { api, .. } => api.prevent_exit(),
            _ => {},
        });
}
