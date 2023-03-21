use crate::repository::command::Runcommand;
use crate::repository::repository::RepositoryTrait;

pub struct Dataround {}
impl Dataround {
    pub fn up(runcommand: Runcommand) -> String {
        if let Ok(stdout) = runcommand.program("docker").args(vec!["run", "--name", "snowtrail-redis", "-p", "6380:6379", "-d", "redis"]).exec() {
            stdout.to_string()
        } else {
            "".to_string()
        }
    }
    
    pub fn down(runcommand: Runcommand) {
        let _ = runcommand.program("docker").args(vec!["stop", "snowtrail-redis"]).exec();
    }
}
