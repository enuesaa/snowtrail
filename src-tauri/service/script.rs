use serde::{Serialize, Deserialize};
use crate::repository::{rocks::RocksRepository, runcommand::RuncommandRepository};
use crate::service::withid::WithId;
use crate::service::crud::Crud;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    id: Option<String>,
    name: Option<String>,
    commands: Option<Vec<String>>,
    project_name: Option<String>,
}
impl Script {
    pub fn new(name: String, commands: Vec<String>, project_name: String) -> Self {
        Script { id: None, name: Some(name), commands: Some(commands), project_name: Some(project_name) }
    }

    pub fn get_name(&self) -> String {
        self.name.clone().unwrap_or("".to_string())
    }
    
    pub fn get_commands(&self) -> Vec<String> {
        self.commands.clone().unwrap_or(vec![])
    }

    pub fn get_project_name(&self) -> String {
        self.project_name.clone().unwrap_or("".to_string())
    }
}
impl WithId for Script {
    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

pub struct ScriptService {
    rocks: RocksRepository,
    runcommand: RuncommandRepository,
}
impl ScriptService {
    pub fn new(rocks: RocksRepository, runcommand: RuncommandRepository) -> Self {
        ScriptService { rocks, runcommand }
    }

    fn runcommand(&self) -> RuncommandRepository {
        self.runcommand.clone()
    }

    pub fn run(&self, name: &str) {
        let script = self.get(name);
        let commands = script.get_commands();
        let mut args: Vec<&str> = commands[0].split(" ").collect();
        println!("{:?}", args);
        args.rotate_left(1);
        let command = args.pop().unwrap();
        let runcommand = self.runcommand().program(command).args(args);
        let res = runcommand.exec();
        println!("{:?}", res);
    }
}

impl Crud<Script> for ScriptService {
    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }
    fn cfname(&self) -> &str {
        "script"
    }
}
