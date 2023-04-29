use serde::{Serialize, Deserialize};
use crate::repository::{rocks::RocksRepository, runcommand::{self, RuncommandRepository}};

#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    name: Option<String>,
    commands: Option<Vec<String>>,
    project_name: Option<String>,
}
impl Script {
    pub fn new(name: String, commands: Vec<String>, project_name: String) -> Self {
        Script { name: Some(name), commands: Some(commands), project_name: Some(project_name) }
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

#[derive(Serialize, Deserialize)]
pub struct ProjectScript {
    project_name: Option<String>,
    script_names: Option<Vec<String>>,
}
impl ProjectScript {
    pub fn new(project_name: String) -> Self {
        ProjectScript { project_name: Some(project_name), script_names: Some(vec![]) }
    }

    pub fn add_script(&mut self, script_name: String) {
        let mut script_names = self.script_names.clone().unwrap_or(vec![]);
        script_names.push(script_name);
        self.script_names = Some(script_names);
    }

    pub fn remove_script(&mut self, script_name: String) {
        let mut script_names: Vec<String> = self.script_names.clone().unwrap_or(vec![]);
        script_names = script_names.iter().filter(|&s| s != &script_name).cloned().collect();
        self.script_names = Some(script_names);
    }

    pub fn list_script_names(&self) -> Vec<String> {
        self.script_names.clone().unwrap_or(vec![])
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

    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
    }

    fn runcommand(&self) -> RuncommandRepository {
        self.runcommand.clone()
    }

    pub fn list(&self) -> Vec<Script> {
        let kvs = self.rocks().list("script", "", 100);
        let mut list: Vec<Script> = vec![];
        for kv in kvs {
            list.push(serde_json::from_str(&kv.value).unwrap());
        };
        list
    }

    pub fn list_in_project(&self, project_name: String) -> Vec<Script> {
        let res = self.rocks().get("project_script", &project_name);
        let binding: ProjectScript = serde_json::from_str(&res.value).unwrap();
        let names = binding.list_script_names();
        let mut list: Vec<Script> = vec![];
        for name in names {
            let script = self.rocks().get("script", &name);
            list.push(serde_json::from_str(&script.value).unwrap());
        };
        list
    }

    pub fn get(&self, name: &str) -> Script {
        let res = self.rocks().get("script", name);
        serde_json::from_str(&res.value).unwrap()
    }

    pub fn create(&self, script: Script) {
        self.rocks().put("script", &script.get_name(), &serde_json::to_string(&script).unwrap());
        let res = self.rocks().get("project_script", &script.get_project_name());
        let mut binding: ProjectScript = serde_json::from_str(&res.value).unwrap();
        binding.add_script(script.get_name());
        self.rocks().put("project_script", &script.get_project_name(), &serde_json::to_string(&binding).unwrap());
    }

    pub fn delete(&self, script: Script) {
        self.rocks().delete("script", &script.get_name());
        let res = self.rocks().get("project_script", &script.get_project_name());
        let mut binding: ProjectScript = serde_json::from_str(&res.value).unwrap();
        binding.remove_script(script.get_name());
        self.rocks().put("project_script", &script.get_project_name(), &serde_json::to_string(&binding).unwrap());
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
