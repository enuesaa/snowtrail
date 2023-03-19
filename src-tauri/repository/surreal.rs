use std::error::Error;
use crate::repository::repository::RepositoryTrait;
use surrealdb::Surreal as BaseSurreal;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;

pub struct Surreal {}
impl Surreal {
    pub fn new() -> Self {
        Surreal {}
    }
}

impl RepositoryTrait<BaseSurreal<Client>> for Surreal {
    fn exec(self) -> Result<BaseSurreal<Client>, Box<dyn Error>> {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let db = BaseSurreal::new::<Http>("localhost:8000").await?;
            let _ = db.signin(Root { username: "root", password: "root" }).await;
            let _ = db.use_ns("test").use_db("test").await;
            Ok(db)
        })
    }
}