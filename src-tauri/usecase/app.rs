use serde::{Serialize, Deserialize};

use crate::repository::runcommand::RuncommandRepository;
use std::fs::File;
use std::io::{BufWriter, Write};

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

    pub fn savejson(&self) {
        let config = Config {
            name: "hey".to_string(),
            is_ok: true,
        };
        if let Ok(file) = File::create("test.json") {
            let mut writer = BufWriter::new(file);
            let _ = serde_json::to_writer_pretty(&mut writer, &config);
            let _ = writer.flush();
        }
    }
}
