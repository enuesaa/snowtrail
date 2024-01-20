use crate::repository::runcommand::RuncommandRepository;

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
}
