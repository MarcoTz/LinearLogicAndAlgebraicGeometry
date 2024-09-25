use super::{
    deduction::Rule,
    formula::{Formula, Sequent},
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    ActiveNotFound(Formula, Sequent),
    CannotApply(Rule),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ActiveNotFound(form, s) => {
                write!(f, "Could not find formula {form} in sequent {s}")
            }
            Error::CannotApply(rule) => write!(f, "Cannot apply rule {rule}"),
        }
    }
}

impl std::error::Error for Error {}
