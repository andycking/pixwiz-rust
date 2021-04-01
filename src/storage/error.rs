use std::error::Error;

#[derive(Debug)]
pub struct StorageError;

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Error for StorageError {}
