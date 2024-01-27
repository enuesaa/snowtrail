use std::fs::File;

use crate::usecase::app::{AppUsecase, ScriptSchema};

pub async fn runasync(script: ScriptSchema) -> Option<String> {
    println!("called");
    let _ = File::create("a.log");
    let mut appcase = AppUsecase::new();
    let _ = appcase.run_script(script);

    Some("response".into())
}
