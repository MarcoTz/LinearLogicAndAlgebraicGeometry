use super::{AttachContext, AttachToNet, ProofLink};
use crate::{
    directed_multigraph::{DirectedMultiGraph, GraphVertex},
    errors::Error,
    proof_structure::{ProofStructure, RuleLabel},
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
        let new_vert = net.add_vertex(net.fresh_label(RuleLabel::Cut))?;
        net.add_edge(
            &left.get_label(),
            &new_vert.get_label(),
            self.premise.clone().neg(),
        )?;
        net.add_edge(&right.get_label(), &new_vert.get_label(), self.premise)?;
        Ok(())
    }
}
