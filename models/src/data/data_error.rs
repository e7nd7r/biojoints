use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum DataError {
    NotInsertedEntity(String),
    QueryError(String),
    AlreadyExist(String),
    UnknownError(String),
    UnexpectedResult(String),
}

impl Display for DataError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

