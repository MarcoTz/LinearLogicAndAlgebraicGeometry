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
    pub prev_left: Option<&'a Vertex>,
    pub prev_right: Option<&'a Vertex>,
    pub next_left: Option<&'a Vertex>,
    pub next_right: Option<&'a Vertex>,
}

impl<'a> Default for AttachContext<'a> {
    fn default() -> AttachContext<'a> {
        AttachContext {
            prev_left: None,
            prev_right: None,
            next_left: None,
            next_right: None,
        }
    }
}

pub trait AttachToNet {
    fn attach(self, net: &mut ProofStructure, ctx: AttachContext) -> Result<(), Error>;
}