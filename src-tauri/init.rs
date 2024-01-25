use std::io;

use crate::repository::runcommand::RuncommandRepository;
use crate::usecase::app::{AppUsecase, ConfigSchema};
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

pub fn init() -> Result<(), io::Error> {
    RuncommandRepository::init()?;

    let appcase = AppUsecase::new();
    let is = appcase.is_registry_exist()?;
    if !is {
        appcase.create_registry()?;
        let config = ConfigSchema {
            updated: "2024-01-21T15:16:00+09:00".to_string(),
            scripts: vec![],
        };
        appcase.writeconfig(config)?;
    };
    Ok(())
}

pub fn create_menu() -> SystemTrayMenu {
    let appcase = AppUsecase::new();
    let mut menu = SystemTrayMenu::new();
    if let Ok(config) = appcase.readconfig() {
        for script in config.scripts {
            let item = CustomMenuItem::new(script.name.clone(), script.name.clone());
            menu = menu.add_item(item);
        }
    };
    // see https://zenn.dev/izuchy/scraps/b101088f10f806
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let reload = CustomMenuItem::new("reload".to_string(), "Reload");

    menu.add_native_item(SystemTrayMenuItem::Separator)
        .add_item(reload)
        .add_item(quit)
}
