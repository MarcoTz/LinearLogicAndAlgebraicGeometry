use std::fmt;

#[derive(Debug)]
pub enum Error {
    EdgeNotFound(String),
    BadLabel(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::EdgeNotFound(label) => write!(f, "Could not find edge {label}"),
            Error::BadLabel(label) => write!(f, "Unexpected edge label {label}"),
        }
    }
}
impl std::error::Error for Error {}
