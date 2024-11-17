use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AppError {
    UserAlreadyExists,
    UserCreationFailed,
    InvalidCredentials,
    DatabaseOperationFailed,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UserAlreadyExists => write!(f, "User already exists"),
            AppError::UserCreationFailed => write!(f, "Failed to create user"),
            AppError::InvalidCredentials => write!(f, "Invalid credentials"),
            AppError::DatabaseOperationFailed => write!(f, "Database operation failed"),
        }
    }
}

impl Error for AppError {}

impl From<Box<dyn Error>> for AppError {
    fn from(error: Box<dyn Error>) -> Self {
        AppError::DatabaseOperationFailed
    }
}
