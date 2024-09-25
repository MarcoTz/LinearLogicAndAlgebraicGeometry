use super::{formula::Formula, proof::Rule, sequent::Sequent};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    ActiveNotFound(Formula, Sequent),
    CannotApply(Rule),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ActiveNotFound(form, seq) => {
                write!(
                    f,
                    "Active formula {} was not found in sequent {}",
                    form,
                    seq.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Error::CannotApply(rule) => write!(f, "Cannot apply rule {rule}"),
        }
    }
}
