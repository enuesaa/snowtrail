use crate::repository::runcommand::Runcommand;

#[derive(Debug)]
pub struct Shell {
    pub hash: String,
}

#[derive(Debug)]
pub struct ShellService {}
impl ShellService {
    pub fn run(runcommand: Runcommand) {
    }
}