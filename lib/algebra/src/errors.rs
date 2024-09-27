use std::fmt;

#[derive(Debug)]
pub enum Error {
    NotAnElement { elem: String, set: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NotAnElement { elem, set } => write!(f, "{elem} is not an element of {set}"),
        }
    }
}

impl std::error::Error for Error {}
