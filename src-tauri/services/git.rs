use serde::Serialize;
use crate::repository::command::{RunCommand, RunCommander};

#[derive(Debug, Serialize, PartialEq)]
struct GitHistory {
    hash: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct GitHistories {
    items: Vec<GitHistory>,
}
pub fn get_git_histories(runcommand: RunCommand) -> GitHistories {
    let mut ret = GitHistories { items: Vec::new() };
    if let Ok(stdout) = runcommand.program("git").args(vec!["log", "--pretty=format:%H", "-n", "5"]).run() {
        stdout.split("\n").for_each(|v| {
            ret.items.push(GitHistory { hash: v.to_string() })
        });
    }
    ret
}

#[test]
fn test_get_git_histories() {
    impl RunCommand {
        fn run(self) -> Result<String, String> {
            Ok("a".to_string())
        }
    }
    let runcommand = RunCommand::new();
    let histories = get_git_histories(runcommand);
    assert_eq!(GitHistories { items: vec![ GitHistory { hash: "a".to_string() }] }, histories);
}