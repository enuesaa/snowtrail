use crate::repository::command::Runcommand;
use crate::repository::repository::RepositoryTrait;
use dirs;

pub struct Dataround {}
impl Dataround {
    pub fn up(runcommand: Runcommand) -> String {
        if let Ok(stdout) = runcommand.dir(dirs::home_dir().unwrap()).program("docker").args(vec!["run", "--name", "snowtrail-redis", "-p", "6380:6379", "-d", "redis"]).exec() {
                stdout.to_string()
        } else {
            "".to_string()
        }
    }
    
    pub fn down(runcommand: Runcommand) {
        let rmcommand = runcommand.clone();
        let _ = runcommand.dir(dirs::home_dir().unwrap()).program("docker").args(vec!["stop", "snowtrail-redis"]).exec();
        let _ = rmcommand.dir(dirs::home_dir().unwrap()).program("docker").args(vec!["rm", "snowtrail-redis"]).exec();
    }
}
