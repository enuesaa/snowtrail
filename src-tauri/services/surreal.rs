use std::process::{Command, Stdio};
use surrealdb::Surreal;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::Error;

pub fn updb() -> String {
    let output = Command::new("docker")
        .args(["run", "-d", "--rm", "--name", "snowtrail-surreal", "-p", "8000:8000", "surrealdb/surrealdb:latest", "start", "--user", "root", "--pass", "root"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}

pub fn downdb() {
    Command::new("docker")
        .args(["stop", "snowtrail-surreal"])
        .output()
        .unwrap();
}

pub async fn connect() -> Result<Surreal<Client>, Error> {
    let db = Surreal::new::<Http>("localhost:8000").await?;
    db.signin(Root { username: "root", password: "root" }).await?;
    db.use_ns("test").use_db("test").await?;
    Ok(db)
}
