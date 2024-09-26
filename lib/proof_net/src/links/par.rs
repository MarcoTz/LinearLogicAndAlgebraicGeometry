use super::{AttachContext, AttachToNet, ProofLink};
use crate::{
    errors::Error,
    proof_structure::{ProofStructure, VertexLabel},
};
use mll::formula::Formula;
use std::rc::Rc;

pub struct ParLink {
    premise_left: Formula,
    premise_right: Formula,
}

impl ParLink {
    pub fn new(premise_left: Formula, premise_right: Formula) -> ParLink {
        ParLink {
            premise_left,
            premise_right,
        }
    }
}
impl From<ParLink> for ProofLink {
    fn from(par: ParLink) -> ProofLink {
        ProofLink::ParLink(par)
    }
}

impl AttachToNet for ParLink {
    fn attach(self, net: &mut ProofStructure, ctx: AttachContext) -> Result<(), Error> {
        let left = ctx.prev_left.ok_or(Error::MissingPremise)?;
        let right = ctx.prev_right.ok_or(Error::MissingPremise)?;
        let next = ctx.next_left.ok_or(Error::MissingConclusion)?;
        let new_vert = net.add_vertex(VertexLabel::Par);
        net.add_edge(left, &new_vert, self.premise_left.clone())?;
        net.add_edge(right, &new_vert, self.premise_right.clone())?;
        net.add_edge(
            &new_vert,
            next,
            Formula::Par(Rc::new(self.premise_left), Rc::new(self.premise_right)),
        )?;
        Ok(())
    }
}
