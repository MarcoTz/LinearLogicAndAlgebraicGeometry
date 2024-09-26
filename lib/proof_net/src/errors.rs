use super::proof_structure::Vertex;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    VertexNotFound(Vertex),
    BadProof,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::VertexNotFound(v) => write!(f, "Could not find vertex {v}"),
            Error::BadProof => write!(f, "Proof is malformed"),
        }
    }
}
impl std::error::Error for Error {}
