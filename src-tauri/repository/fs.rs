use std::fs;
use std::io;
use std::io::Error;
use std::path::Path;

#[derive(Clone)]
pub struct FsRepository {}

impl FsRepository {
    pub fn new() -> Self {
        FsRepository {}
    }

    pub fn is_exist(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    pub fn is_dir(&self, path: &str) -> Result<bool, Error> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.is_dir())
    }

    pub fn create_dir(&self, path: &str) -> Result<(), Error> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    pub fn create(&self, path: &str, body: Vec<u8>) -> Result<(), Error> {
        fs::write(path, body)?;
        Ok(())
    }

    pub fn read(&self, path: &str) -> Result<Vec<u8>, Error> {
        fs::read(path)
    }

    pub fn homedir(&self) -> Result<String, Error> {
        if let Some(path) = dirs::home_dir() {
            return Ok(path.to_str().unwrap().to_string());
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "failed to find homedir.",
        ))
    }

    pub fn workdir(&self) -> Result<String, Error> {
        let path = std::env::current_dir()?;
        Ok(path.to_str().unwrap().to_string())
    }

    pub fn remove(&self, path: &str) -> Result<(), Error> {
        fs::remove_dir_all(path)?;
        Ok(())
    }
}
