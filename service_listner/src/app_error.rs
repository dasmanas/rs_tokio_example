use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct AppError {
    message: String,
}

impl AppError {
    fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "AppError: {}", self.message)
    }
}

impl Error for AppError {}