use serde::{Serialize, Deserialize};

use crate::repository::fs::{FsRepository, FsRepositoryInterface};
use crate::repository::runcommand::RuncommandRepository;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub scripts: Vec<Script>,
    pub updated: String, // datetime
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub name: String,
    pub command: Vec<String>,
    pub pid: Option<u64>, // also indicate executing
    pub description: String,
}

pub struct AppUsecase {}
impl AppUsecase {
    pub fn new() -> Self {
        AppUsecase {}
    }

    pub fn run_script(&self) -> String {
        let result = RuncommandRepository::new()
            .program("echo")
            .args(vec!["aaa"])
            .exec();

        if let Ok(output) = result {
            output
        } else {
            "err".to_string()
        }
    }

    pub fn get_registrypath(&self) -> Result<String, Error> {
        let fs: FsRepository = FsRepository::new();
        let homedir = fs.homedir()?;
        let path = format!("{}/.snowtrail", homedir);
        Ok(path)
    }

    pub fn get_configpath(&self) -> Result<String, Error> {
        let registrypath = self.get_registrypath()?;
        let configpath = format!("{}/config.json", registrypath);
        Ok(configpath)
    }

    pub fn readconfig(&self) -> Result<Config, Error> {
        let fs: FsRepository = FsRepository::new();
        let configpath = self.get_configpath()?;
        let configbytes = fs.read(&configpath)?;
        let config: Config = serde_json::from_slice(&configbytes)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        Ok(config)
    }

    pub fn writeconfig(&self, config: Config) -> Result<(), Error> {
        let fs = FsRepository::new();
        let registrypath = self.get_registrypath()?;
        fs.create_dir(&registrypath)?;

        let configpath = self.get_configpath()?;
        let configbytes = serde_json::to_vec_pretty(&config)?;
        fs.create(&configpath, configbytes)?;

        Ok(())
    }

    pub fn add_script(&self, script: Script) -> Result<(), Error> {
        let mut config = self.readconfig()?;
        config.scripts.push(script);

        self.writeconfig(config)?;
        Ok(())
    }

    pub fn remove_script(&self, name: String) -> Result<(), Error> {
        let mut config = self.readconfig()?;
        config.scripts = config.scripts.iter()
            .filter(|s| s.name != name)
            .cloned()
            .collect();
        self.writeconfig(config)?;
        Ok(())
    }
}
