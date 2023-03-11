use std::process::{Command, Stdio};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct StartSurrealRepsonse {
    status: String,
}
#[tauri::command]
pub fn start_surreal() -> StartSurrealRepsonse {
    let output = Command::new("docker")
        .args(["run", "-d", "--rm", "--name", "snowtrail-surreal", "-p", "8000:8000", "surrealdb/surrealdb:latest", "start"])
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


use std::borrow::Cow;
use surrealdb::Surreal;
use surrealdb::engine::remote::http::Http;
use surrealdb::opt::auth::Root;

#[derive(Serialize, Deserialize)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

#[derive(Serialize, Deserialize)]
struct Person {
    #[serde(skip_serializing)]
    id: Option<String>,
    title: Cow<'static, str>,
    name: Name,
    marketing: bool,
}

async fn record_async() -> surrealdb::Result<()> {
    let db = Surreal::new::<Http>("localhost:8000").await?;
    db.signin(Root { username: "root", password: "root" }).await?;
    db.use_ns("namespace").use_db("database").await?;

    db.create("person").content(Person {
        id: None,
        title: "titl".into(),
        name: Name {
            first: "bbb".into(),
            last: "aaa".into(),
        },
        marketing: true,
    }).await?;
    Ok(())
}

#[tauri::command]
pub fn record() {
    let res = tauri::async_runtime::block_on(record_async());
    println!("{:?}", res);
}
