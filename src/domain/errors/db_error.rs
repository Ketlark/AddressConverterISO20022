use std::error::Error;
use std::fmt;

use tokio::io;

#[derive(Debug)]
pub enum DbError {
    IoError(String, io::Error),            // Stores a message + original error
    SerdeError(String, serde_json::Error), // Stores a message + original error
}

impl From<io::Error> for DbError {
    fn from(error: io::Error) -> Self {
        DbError::IoError("IO operation failed".to_string(), error)
    }
}

impl From<serde_json::Error> for DbError {
    fn from(error: serde_json::Error) -> Self {
        DbError::SerdeError("JSON parsing failed".to_string(), error)
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::IoError(msg, e) => write!(f, "{}: {}", msg, e),
            DbError::SerdeError(msg, e) => write!(f, "{}: {}", msg, e),
        }
    }
}

impl std::error::Error for DbError {}
