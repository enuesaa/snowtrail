use std::error::Error;
use crate::repository::repository::RepositoryTrait;

pub struct Redis {}
impl Redis {
    pub fn new() -> Self {
        Redis {}
    }
}

impl RepositoryTrait<()> for Redis {
    fn exec(self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}