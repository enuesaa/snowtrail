use crate::repository::runcommand::RuncommandRepository;

pub struct AppUsecase {
    runcommand: RuncommandRepository,
}
impl AppUsecase {
    pub fn new() -> Self {
        AppUsecase {
            runcommand: RuncommandRepository::new(),
        }
    }

    fn runcommand(&self) -> RuncommandRepository {
        self.runcommand.clone()
    }

    pub fn run_script(&self, id: &str) {
        println!("hello");
    }
}
