use serde::Serialize;
use crate::repository::command::Runcommand;
#[allow(unused_imports)]
use crate::repository::repository::RepositoryTrait;

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

#[test]
fn test_fetch_git_histories() {
    impl Runcommand {
        fn exec(self) -> Result<String, String> {
            Ok("a".to_string())
        }
    }
    let histories = GitHistories::fetch(Runcommand::new());
    assert_eq!(GitHistories { items: vec![ GitHistory { hash: "a".to_string() }] }, histories);
}