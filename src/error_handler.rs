use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct AppError {
    error_msg: String,
}

impl AppError {
    pub fn new(error_msg: &str) -> Box<Self> {
        Box::new(Self {
            error_msg: error_msg.to_owned(),
        })
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred: {}", self.error_msg)
    }
}

impl Error for AppError {}
