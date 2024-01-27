use serde::{Deserialize, Serialize};

use crate::repository::fs::FsRepository;
use crate::repository::runcommand::RuncommandRepository;
use std::io;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConfigSchema {
    pub scripts: Vec<ScriptSchema>,
    pub updated: String, // datetime
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptSchema {
    pub name: String,
    pub command: String,
    // pub pid: Option<u64>, // also indicate executing
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogSchema {
    pub name: String,
    pub content: String,
    pub time: String,
}

pub struct AppUsecase {}

impl AppUsecase {
    pub fn new() -> Self {
        AppUsecase {}
    }

    pub async fn run_script(&mut self, script: ScriptSchema) -> Result<(), io::Error> {
        let cmdargs: Vec<&str> = script.command.split(" ").collect();
        if cmdargs.len() < 2 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "invalid command",
            ));
        };

        let mut args = cmdargs.clone();
        let cmd = args.remove(0);
        let out = RuncommandRepository::new().program(cmd).args(args).exec()?;
        let log = LogSchema {
            name: script.name,
            content: out,
            time: Utc::now().to_string(),
        };
        self.write_log(log)?;
        Ok(())
    }

    pub fn write_log(&self, log: LogSchema) -> Result<(), io::Error> {
        self.create_logdir()?;

        let fs = FsRepository::new();
        let logfilepath = self.get_logfilepath()?;
        let logbytes = serde_json::to_vec_pretty(&log)?;
        fs.create(&logfilepath, logbytes)?;
        Ok(())
    }

    pub fn get_registrypath(&self) -> Result<String, io::Error> {
        let fs: FsRepository = FsRepository::new();
        let homedir = fs.homedir()?;
        let path = format!("{}/.snowtrail", homedir);
        Ok(path)
    }

    pub fn is_registry_exist(&self) -> Result<bool, io::Error> {
        let fs: FsRepository = FsRepository::new();
        let registrypath = self.get_registrypath()?;
        if fs.is_exist(&registrypath) {
            return Ok(true);
        };
        Ok(false)
    }

    pub fn get_logfilepath(&self) -> Result<String, io::Error> {
        let logdirpath = self.get_logdirpath()?;
        let now = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let logfilepath = format!("{}/{}.json", logdirpath, now);
        Ok(logfilepath)
    }
    
    pub fn get_logdirpath(&self) -> Result<String, io::Error> {
        let registrypath = self.get_registrypath()?;
        let path = format!("{}/log", registrypath);
        Ok(path)
    }

    pub fn list_logs(&self) -> Result<Vec<String>, io::Error> {
        let fs: FsRepository = FsRepository::new();
        let logdir = self.get_logdirpath()?;
        let logs = fs.list_filenames(&logdir)?;
        let mut lognames: Vec<String> = vec![];
        for logfilename in logs {
            lognames.push(logfilename.replace(".json", ""));
        }
        Ok(lognames)
    }

    pub fn read_log(&self, name: String) -> Result<LogSchema, io::Error> {
        let fs: FsRepository = FsRepository::new();
        let logdir = self.get_logdirpath()?;
        let logpath = format!("{}/{}.json", logdir, name);
        let logbytes = fs.read(&logpath)?;
        let log: LogSchema = serde_json::from_slice(&logbytes)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(log)
    }

    pub fn create_logdir(&self) -> Result<(), io::Error> {
        let fs = FsRepository::new();
        let path = self.get_logdirpath()?;
        fs.create_dir(&path)?;
        Ok(())
    }

    pub fn create_registry(&self) -> Result<(), io::Error> {
        let fs = FsRepository::new();
        let registrypath = self.get_registrypath()?;
        fs.create_dir(&registrypath)?;
        Ok(())
    }

    pub fn get_configpath(&self) -> Result<String, io::Error> {
        let registrypath = self.get_registrypath()?;
        let configpath = format!("{}/config.json", registrypath);
        Ok(configpath)
    }

    pub fn readconfig(&self) -> Result<ConfigSchema, io::Error> {
        let fs: FsRepository = FsRepository::new();
        let configpath = self.get_configpath()?;
        let configbytes = fs.read(&configpath)?;
        let config: ConfigSchema = serde_json::from_slice(&configbytes)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(config)
    }

    pub fn writeconfig(&self, config: ConfigSchema) -> Result<(), io::Error> {
        self.create_registry()?;

        let fs = FsRepository::new();
        let configpath = self.get_configpath()?;
        let configbytes = serde_json::to_vec_pretty(&config)?;
        fs.create(&configpath, configbytes)?;
        Ok(())
    }

    pub fn add_script(&self, script: ScriptSchema) -> Result<(), io::Error> {
        let mut config = self.readconfig()?;
        config.scripts.push(script);

        self.writeconfig(config)?;
        Ok(())
    }

    pub fn get_script(&self, name: String) -> Result<ScriptSchema, io::Error> {
        let config = self.readconfig()?;
        for script in config.scripts {
            if script.name == name {
                return Ok(script);
            };
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "failed to find script.",
        ))
    }

    pub fn remove_script(&self, name: String) -> Result<(), io::Error> {
        let mut config = self.readconfig()?;
        config.scripts = config
            .scripts
            .iter()
            .filter(|s| s.name != name)
            .cloned()
            .collect();
        self.writeconfig(config)?;
        Ok(())
    }
}
