use std::fs;
use std::io;
use std::io::Error;
use std::path::Path;

trait FsRepositoryInterface {
    fn is_exist(&self, path: &str) -> bool;
    fn is_dir(&self, path: &str) -> Result<bool, Error>;
    fn create_dir(&self, path: &str) -> Result<(), Error>;
    fn create(&self, path: &str, body: &[u8]) -> Result<(), Error>;
    fn homedir(&self) -> Result<String, Error>;
    fn workdir(&self) -> Result<String, Error>;
    fn remove(&self, path: &str) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct FsRepository {}

impl FsRepositoryInterface for FsRepository {
    fn is_exist(&self, path: &str) -> bool {
        Path::new(path).exists()
    }

    fn is_dir(&self, path: &str) -> Result<bool, Error> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.is_dir())
    }

    fn create_dir(&self, path: &str) -> Result<(), Error> {
        fs::create_dir_all(path)?;
        Ok(())
    }

    fn create(&self, path: &str, body: &[u8]) -> Result<(), Error> {
        fs::write(path, body)?;
        Ok(())
    }

    fn homedir(&self) -> Result<String, Error> {
        if let Some(path) = dirs::home_dir() {
            return Ok(path.to_str().unwrap().to_string())
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "failed to find homedir."))
    }

    fn workdir(&self) -> Result<String, Error> {
        let path = std::env::current_dir()?;
        Ok(path.to_str().unwrap().to_string())
    }

    fn remove(&self, path: &str) -> Result<(), Error> {
        fs::remove_dir_all(path)?;
        Ok(())
    }
}
