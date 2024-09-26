use super::{AttachContext, AttachToNet, ProofLink};
use crate::{
    errors::Error,
    proof_structure::{ProofStructure, VertexLabel},
};
use mll::formula::Formula;

pub struct ConclusionLink {
    conclusion: Formula,
}

impl ConclusionLink {
    pub fn new(conclusion: Formula) -> ConclusionLink {
        ConclusionLink { conclusion }
    }
}

impl From<ConclusionLink> for ProofLink {
    fn from(conc: ConclusionLink) -> ProofLink {
        ProofLink::ConclusionLink(conc)
    }
}

impl AttachToNet for ConclusionLink {
    fn attach(self, net: &mut ProofStructure, ctx: AttachContext) -> Result<(), Error> {
        let new_vert = net.add_vertex(VertexLabel::C);
        let prev = ctx.prev_left.ok_or(Error::MissingPremise)?;
        net.add_edge(prev, &new_vert, self.conclusion)?;
        Ok(())
    }
}
