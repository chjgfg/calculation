use std::{
    fmt::{Debug, Display},
    io, num,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone)]
pub enum Error {
    IO(String),
    Parse(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(s) | Error::Parse(s) => write!(f, "{}", s),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::IO(value.to_string())
    }
}

impl From<num::ParseFloatError> for Error {
    fn from(value: num::ParseFloatError) -> Self {
        Error::Parse(value.to_string())
    }
}
