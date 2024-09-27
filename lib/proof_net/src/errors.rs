use super::proof_structure::{RuleLabel, VertexLabel};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    VertexNotFound(VertexLabel),
    BadProof,
    MissingPremise,
    MissingConclusion,
    WrongLabel {
        found: RuleLabel,
        expected: RuleLabel,
    },
    VertexAlreadyExists(VertexLabel),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VertexNotFound(v) => write!(f, "Could not find vertex {v}"),
            Error::BadProof => write!(f, "Proof is malformed"),
            Error::MissingPremise => write!(f, "Expected premise, but found none"),
            Error::MissingConclusion => write!(f, "Expected conclusion, but found none"),
            Error::WrongLabel { found, expected } => {
                write!(f, "Unexpected label {found}, expected {expected}")
            }
            Error::VertexAlreadyExists(label) => {
                write!(f, "Vertex with label {label} already exists")
            }
        }
    }
}
impl std::error::Error for Error {}
