use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct InvalidConfigError {
    pub msg: String,
}

impl InvalidConfigError {
    pub fn new(msg: &String) -> InvalidConfigError {
        InvalidConfigError { msg: msg.clone() }
    }
}

impl Display for InvalidConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "InvalidConfigError: {}", self.msg)
    }
}

impl Error for InvalidConfigError {}
