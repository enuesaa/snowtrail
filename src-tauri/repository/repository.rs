use std::error::Error;

pub trait RepositoryTrait<R> {
    fn exec(self) -> Result<R, Box<dyn Error>>;
}