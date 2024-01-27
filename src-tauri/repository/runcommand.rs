use std::fs::File;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::{env, io, io::Write};

#[derive(Clone)]
pub struct RuncommandRepository {
    program: String,
    args: Vec<String>,
    dir: PathBuf,
}

impl RuncommandRepository {
    pub fn init() -> Result<(), io::Error> {
        fix_path_env::fix().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(())
    }

    pub fn new() -> Self {
        RuncommandRepository {
            program: "".to_string(),
            args: vec![],
            dir: env::current_dir().unwrap(),
        }
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

    pub fn exec(self) -> Result<(), io::Error> {
        let output = Command::new(self.program)
            .args(self.args)
            .current_dir(self.dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()?;

        let mut file = File::create("a.log")?;
        let outstr = String::from_utf8_lossy(&output.stdout);
        writeln!(file, "{}", outstr)?;
        Ok(())
    }

    pub fn log(self, child: Child) -> Result<(), io::Error> {
        if let Some(mut stdout) = child.stdout {
            let mut file = File::create("a.log")?;
            io::copy(&mut stdout, &mut file)?;
        };
        Ok(())
    }
}
