use std::{fmt, io};

#[derive(Debug)]
pub struct AocError {
    message: String,
}

impl AocError {
    pub fn new(message: &str) -> AocError {
        AocError { message: message.to_owned() }
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