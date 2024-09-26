use super::sequent::Sequent;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    SequentMismatch(Sequent, Sequent),
    MissingPremise(Sequent),
    WrongNumberOfPremises { expected: i32, found: i32 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::SequentMismatch(seq1, seq2) => {
                write!(
                    f,
                    "Sequents {} and {} should be equal",
                    seq1.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<String>>()
                        .join(", "),
                    seq2.iter()
                        .map(|f| format!("{}", f))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Error::MissingPremise(s) => write!(
                f,
                "Missing premise {}",
                s.iter()
                    .map(|f| format!("{}", f))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Error::WrongNumberOfPremises { expected, found } => write!(
                f,
                "Wrong number of premises, expected {expected}, found {found}"
            ),
        }
    }
}
