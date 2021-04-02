use std::error::Error;

#[derive(Debug)]
pub struct StorageError;

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for StorageError {}

impl StorageError {
    pub fn new() -> Self {
        Self {}
    }
}

impl From<std::io::Error> for StorageError {
    fn from(_: std::io::Error) -> Self {
        Self::new()
    }
}
