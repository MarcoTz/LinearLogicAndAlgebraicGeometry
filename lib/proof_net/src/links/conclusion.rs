use super::{AttachContext, AttachToNet, ProofLink};
use crate::{
    directed_multigraph::{DirectedMultiGraph, GraphVertex},
    errors::Error,
    proof_structure::{ProofStructure, RuleLabel},
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
        let new_vert = net.add_vertex(net.fresh_label(RuleLabel::C))?;
        let prev = ctx.prev_left.ok_or(Error::MissingPremise)?;
        net.add_edge(&prev.get_label(), &new_vert.get_label(), self.conclusion)?;
        Ok(())
    }
}
