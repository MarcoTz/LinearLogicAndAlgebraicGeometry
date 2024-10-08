use std::fmt;

#[derive(Debug)]
pub enum Error {
    ProjectiveAllZero,
    DivisionByZero,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ProjectiveAllZero => {
                f.write_str("Projective points cannot have all zero coordinates")
            }
            Error::DivisionByZero => f.write_str("Cannot divide by zero"),
        }
    }
}
