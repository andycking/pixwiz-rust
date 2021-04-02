use std::error::Error;

/// Wrap various storage-specific errors, like PNG encoding/decoding errors.
#[derive(Debug)]
pub struct StorageError;

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for StorageError {}

impl From<std::io::Error> for StorageError {
    fn from(_: std::io::Error) -> Self {
        Self::new()
    }
}

impl StorageError {
    pub fn new() -> Self {
        Self {}
    }
}
