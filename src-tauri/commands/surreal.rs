use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct StartSurrealRepsonse {
    status: String,
}
#[tauri::command]
pub fn start_surreal() -> StartSurrealRepsonse {
    let output = Command::new("docker")
        .args(["run", "-d", "--rm", "--name", "snowtrail-surreal", "-p", "8000:8000", "surrealdb/surrealdb:latest", "start", "--user", "root", "--pass", "root"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .unwrap();
    StartSurrealRepsonse { status: String::from_utf8(output.stdout).unwrap() }
}

#[tauri::command]
pub fn end_surreal() {
    Command::new("docker")
        .args(["stop", "snowtrail-surreal"])
        .output()
        .unwrap();
}


use surrealdb::Surreal;
use surrealdb::engine::remote::http::Http;
use surrealdb::opt::auth::Root;

#[derive(Serialize, Deserialize)]
struct Person {
    title: String,
    marketing: bool,
}

async fn record_async() -> surrealdb::Result<()> {
    let db = Surreal::new::<Http>("localhost:8000").await?;
    if let Err(e) = db.signin(Root { username: "root", password: "root" }).await {
        println!("{:?}", e);
    };
    if let Err(e) = db.use_ns("test").use_db("test").await {
        println!("{:?}", e);
    };
    let person = Person {
        title: "titll".to_string(),
        marketing: true,
    };
    db.create("person").content(person).await?;
    Ok(())
}

#[tauri::command]
pub fn record() {
    let res = tauri::async_runtime::block_on(record_async());
    println!("{:?}", res);
}
