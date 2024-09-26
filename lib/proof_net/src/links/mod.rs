mod ax;
mod conclusion;
mod cut;
mod par;
mod tensor;

use crate::{
    errors::Error,
    proof_structure::{ProofStructure, Vertex},
};
pub use ax::AxLink;
pub use conclusion::ConclusionLink;
pub use cut::CutLink;
pub use par::ParLink;
pub use tensor::TensorLink;

pub enum ProofLink {
    AxLink(AxLink),
    CutLink(CutLink),
    TensorLink(TensorLink),
    ParLink(ParLink),
    ConclusionLink(ConclusionLink),
}

pub struct AttachContext<'a> {
    prev_left: Option<&'a Vertex>,
    prev_right: Option<&'a Vertex>,
    next_left: Option<&'a Vertex>,
    next_right: Option<&'a Vertex>,
}

pub trait AttachToNet {
    fn attach(self, net: &mut ProofStructure, ctx: AttachContext) -> Result<(), Error>;
}
