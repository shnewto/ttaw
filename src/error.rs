use reqwest;
use serde_json;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InputError(String),
    ProgramError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Error::ParseError(ref s) => write!(f, "{}", s),
            Error::InputError(ref s) => write!(f, "{}", s),
            Error::ProgramError(ref s) => write!(f, "{}", s),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::InputError(format!("{}", err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::InputError(format!("{}", err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::InputError(format!("{}", err))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display() {
        let input = "input error".to_string();
        let progam = "program error".to_string();

        assert_eq!(input, format!("{}", Error::InputError(input.clone())));
        assert_eq!(progam, format!("{}", Error::ProgramError(progam.clone())));
    }

    #[test]
    fn io_err() {
        let err_str = "IO Errored!";
        let error = std::io::Error::new(std::io::ErrorKind::Other, err_str);

        assert_eq!(err_str.to_string(), format!("{}", Error::from(error)));
    }
}
