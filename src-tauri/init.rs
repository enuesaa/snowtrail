use std::io;

use crate::repository::runcommand::RuncommandRepository;
use crate::usecase::app::{AppUsecase, ConfigSchema};

pub fn init() -> Result<(), io::Error>{
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
