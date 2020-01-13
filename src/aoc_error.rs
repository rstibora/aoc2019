use std::{fmt, io};

#[derive(Debug)]
pub struct AocError {
    message: String,
}

pub type AocResult = Result<String, AocError>;

impl AocError {
    pub fn new(message: String) -> AocError {
        AocError { message }
    }
}

impl fmt::Display for AocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for AocError {
    fn from(error: io::Error) -> Self {
        AocError { message: format!("IO error: {}", error.to_string()) }
    }
}