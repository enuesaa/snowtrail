use serde::{Serialize, Deserialize};

use crate::repository::fs::{FsRepository, FsRepositoryInterface};
use crate::repository::runcommand::RuncommandRepository;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Config {
    scripts: Vec<Script>,
    updated: String, // datetime
}

#[derive(Serialize, Deserialize, Debug)]
struct Script {
    name: String,
    command: Vec<String>,
    pid: Option<u64>, // also indicate executing
    description: String,
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

    pub fn savejson(&self) -> Result<(), Error> {
        let fs = FsRepository::new();
        let homedir = fs.homedir()?;
        let path = format!("{}/.snowtrail", homedir);
        println!("path: {}", path);
        let _ = fs.create_dir(&path)?;

        let config = Config {
            updated: "2024-01-21T15:16:00+09:00".to_string(),
            scripts: vec![
                Script {
                    name: "example".to_string(),
                    command: vec!["echo".to_string(), "aa".to_string()],
                    description: "example example".to_string(),
                    pid: None,
                },
                Script {
                    name: "example2".to_string(),
                    command: vec!["echo".to_string(), "bb".to_string()],
                    description: "example2".to_string(),
                    pid: None,
                },
            ],
        };

        let configpath = format!("{}/config.json", path);
        let configbytes = serde_json::to_vec_pretty(&config)?;
        let _ = fs.create(&configpath, configbytes)?;

        Ok(())
    }
}
