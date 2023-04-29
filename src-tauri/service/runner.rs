use crate::repository::rocks::RocksRepository;
use crate::repository::runcommand::RuncommandRepository;
use crate::service::script::Script;
use crate::service::event::{Event, EventService};

pub struct ScriptRunnerService {
    rocks: RocksRepository,
    runcommand: RuncommandRepository,
}
impl ScriptRunnerService {
    pub fn new(rocks: RocksRepository, runcommand: RuncommandRepository) -> Self {
        ScriptRunnerService { rocks, runcommand }
    }

    fn rocks(&self) -> RocksRepository {
        self.rocks.clone()
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

        let event = Event::new("snowtrail:command:run");
        let id = EventService::create(self.rocks(), event);
        println!("{:?}", id);
    }
}
