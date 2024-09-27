use super::{AttachContext, AttachToNet, ProofLink};
use crate::{
    errors::Error,
    proof_structure::{ProofStructure, VertexLabel},
};
use mll::formula::Formula;
use std::ops::Neg;

pub struct CutLink {
    premise: Formula,
}

impl CutLink {
    pub fn new(premise: Formula) -> CutLink {
        CutLink { premise }
    }
}

impl From<CutLink> for ProofLink {
    fn from(cut: CutLink) -> ProofLink {
        ProofLink::CutLink(cut)
    }
}

impl From<Formula> for CutLink {
    fn from(premise: Formula) -> CutLink {
        CutLink::new(premise)
    }
}

impl AttachToNet for CutLink {
    fn attach(self, net: &mut ProofStructure, ctx: AttachContext) -> Result<(), Error> {
        let left = ctx.prev_left.ok_or(Error::MissingPremise)?;
        let right = ctx.prev_right.ok_or(Error::MissingPremise)?;
        let new_vert = net.add_vertex(VertexLabel::Cut);
        net.add_edge(left, &new_vert, self.premise.clone().neg())?;
        net.add_edge(right, &new_vert, self.premise)?;
        Ok(())
    }
}
