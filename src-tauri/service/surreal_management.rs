use surrealdb::Surreal;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::Error;
use crate::repository::command::Runcommand;
use crate::repository::repository::RepositoryTrait;

pub struct SurrealManagement {}
impl SurrealManagement {
    pub fn up(runcommand: Runcommand) -> String {
        if let Ok(stdout) = runcommand.program("docker").args(vec!["run", "-d", "--rm", "--name", "snowtrail-surreal", "-p", "8000:8000", "surrealdb/surrealdb:latest", "start", "--user", "root", "--pass", "root"]).exec() {
            stdout.to_string()
        } else {
            "".to_string()
        }
    }
    
    pub fn down(runcommand: Runcommand) {
        let _ = runcommand.program("docker").args(vec!["stop", "snowtrail-surreal"]).exec();
    }
}
