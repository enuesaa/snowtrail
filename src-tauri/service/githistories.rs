use serde::Serialize;
use crate::repository::runcommand::Runcommand;

#[derive(Debug, Serialize, PartialEq)]
pub struct GitHistory {
    pub hash: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct GitHistories {
    pub items: Vec<GitHistory>,
}
impl GitHistories {
    pub fn new() -> Self {
        GitHistories { items: vec![] }
    }

    pub fn fetch(runcommand: Runcommand) -> Self {
        let mut histories = GitHistories { items: vec![] };
        if let Ok(stdout) = runcommand.program("git").args(vec!["log", "--pretty=format:%H", "-n", "5"]).exec() {
            stdout.split("\n").for_each(|v| {
                histories.items.push(GitHistory { hash: v.to_string() })
            });
        };
        histories
    }
}
