use super::proof_structure::Vertex;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    VertexNotFound(Vertex),
    BadProof,
    MissingPremise,
    MissingConclusion,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VertexNotFound(v) => write!(f, "Could not find vertex {v}"),
            Error::BadProof => write!(f, "Proof is malformed"),
            Error::MissingPremise => write!(f, "Expected premise, but found none"),
            Error::MissingConclusion => write!(f, "Expected conclusion, but found none"),
        }
    }
}
impl std::error::Error for Error {}
