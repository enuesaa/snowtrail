use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::env;
use std::error::Error;

#[derive(Clone)]
pub struct Runcommand {
    program: String,
    args: Vec<String>,
    dir: PathBuf,
}

impl Runcommand {
    pub fn initialize() {
        let _ = fix_path_env::fix();
    }

    pub fn new() -> Self {
        Runcommand { program: "".to_string(), args: vec![], dir: env::current_dir().unwrap() }
    }

    pub fn program(mut self, program: &str) -> Self {
        self.program = program.to_string();
        self
    }

    pub fn args(mut self, args: Vec<&str>) -> Self {
        self.args = args.iter().map(|&v| v.to_string()).collect();
        self
    }

    pub fn dir(mut self, dir: PathBuf) -> Self {
        self.dir = dir;
        self
    }

    pub fn exec(self) -> Result<String, Box<dyn Error>> {
        let output = Command::new(self.program)
            .args(self.args)
            .current_dir(self.dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        Ok(String::from_utf8(output.stdout).unwrap())
    }
}
