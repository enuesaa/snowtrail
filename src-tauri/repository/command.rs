use std::process::{Command, Stdio};
use std::env;
use std::error::Error;

pub trait RunCommander<T> {
    fn program(self, program: &str) -> T;
    fn args(self, args: Vec<&str>) -> T;
    fn run(self) -> Result<String, Box<dyn Error>>;
}

pub struct RunCommand {
    program: String,
    args: Vec<String>,
}

impl RunCommand {
    pub fn new() -> Self {
        RunCommand { program: "".to_string(), args: vec![] }
    }
}

impl RunCommander<RunCommand> for RunCommand {
    fn program(mut self, program: &str) -> Self {
        self.program = program.to_string();
        self
    }

    fn args(mut self, args: Vec<&str>) -> Self {
        self.args = args.iter().map(|&v| v.to_string()).collect();
        self
    }

    fn run(self) -> Result<String, Box<dyn Error>> {
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

