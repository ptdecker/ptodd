//! Time module custom errors

use super::*;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidMonth,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidMonth => write!(f, "invalid month value"),
        }
    }
}

impl std::error::Error for Error {}
