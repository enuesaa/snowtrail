use serde::{Serialize, Deserialize};

use crate::repository::fs::{FsRepository, FsRepositoryInterface};
// use crate::repository::fs::FsRepositoryInterface;
use crate::repository::runcommand::RuncommandRepository;
use std::fs::File;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Config {
    name: String,
    is_ok: bool,
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
            name: "hey".to_string(),
            is_ok: true,
        };

        let configpath = format!("{}/config.json", path);
        let configbytes = serde_json::to_vec_pretty(&config)?;
        let _ = fs.create(&configpath, configbytes)?;

        Ok(())
    }
}
