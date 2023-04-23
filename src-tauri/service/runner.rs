use crate::repository::runcommand::RuncommandRepository;
use crate::service::script::Script;

pub struct ScriptRunnerService {
    runcommand: RuncommandRepository,
}
impl ScriptRunnerService {
    pub fn new(runcommand: RuncommandRepository) -> Self {
        ScriptRunnerService { runcommand }
    }

    fn runcommand(&self) -> RuncommandRepository {
        self.runcommand.clone()
    }

    pub fn run(&self, script: Script) {
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
