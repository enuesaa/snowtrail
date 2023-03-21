use std::process::{Command, Stdio};
use std::env;
use std::error::Error;
use crate::repository::repository::RepositoryTrait;

#[derive(Clone)]
pub struct Runcommand {
    program: String,
    args: Vec<String>,
}

impl Runcommand {
    pub fn new() -> Self {
        Runcommand { program: "".to_string(), args: vec![] }
    }

    pub fn program(mut self, program: &str) -> Self {
        self.program = program.to_string();
        self
    }

    pub fn args(mut self, args: Vec<&str>) -> Self {
        self.args = args.iter().map(|&v| v.to_string()).collect();
        self
    }
}

impl RepositoryTrait<String> for Runcommand {
    fn exec(self) -> Result<String, Box<dyn Error>> {
        let output = Command::new(self.program)
            .args(self.args)
            .current_dir(env::current_dir().unwrap())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .unwrap();
        Ok(String::from_utf8(output.stdout).unwrap())
    }
}
