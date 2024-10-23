use std::fmt;

#[derive(Debug)]
pub enum Error {
    ProjectiveAllZero,
    DivisionByZero,
    DimensionMismatch { found: usize, expected: usize },
    WrongDegree { found: usize, expected: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ProjectiveAllZero => {
                f.write_str("Projective points cannot have all zero coordinates")
            }
            Error::DivisionByZero => f.write_str("Cannot divide by zero"),
            Error::DimensionMismatch { found, expected } => {
                write!(f, "Expected dimension {expected}, but got {found}")
            }
            Error::WrongDegree { found, expected } => {
                write!(f, "Expected degree {expected}, got {found}")
            }
        }
    }
}

impl std::error::Error for Error {}
