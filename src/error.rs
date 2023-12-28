//! Custom Errors for ptodd top-level library
use crate::pool;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum Error {
    Pool(String),
    IO(io::Error),
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<pool::Error> for Error {
    fn from(err: pool::Error) -> Self {
        Error::Pool(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Pool(s) => write!(f, "pool error: {}", s),
            Error::IO(e) => write!(f, "io error: {}", e),
        }
    }
}
