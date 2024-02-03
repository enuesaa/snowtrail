use crate::usecase::app::AppUsecase;
use tauri::{AppHandle, CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, Manager};

pub fn create() -> SystemTrayMenu {
    let appcase = AppUsecase::new();
    let mut menu = SystemTrayMenu::new();
    if let Ok(config) = appcase.readconfig() {
        for script in config.scripts {
            let item = CustomMenuItem::new(script.name.clone(), script.name.clone());
            menu = menu.add_item(item);
        }
    };
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let open = CustomMenuItem::new("open".to_string(), "Open");

    menu.add_native_item(SystemTrayMenuItem::Separator)
        .add_item(open)
        .add_item(quit)
}

pub fn handleclick(app: &AppHandle, id: &str) {
    match id {
        "open" => {
            if let Some(window) = app.get_window("main") {
                window.show().unwrap();
            };
        },
        "quit" => {
            std::process::exit(0);
        },
        _ => {
            let mut appcase = AppUsecase::new();
            if let Ok(script) = appcase.get_script(id.to_string().clone()) {
                let item = app.tray_handle().get_item(&id);
                item.set_title("Running").unwrap();
                tokio::spawn(async move {
                    appcase.run_script(script).await.unwrap();
                    item.set_title("OK").unwrap();
                });
            } else {
                println!("Error: no such script.");
            };
        },
    };
}
