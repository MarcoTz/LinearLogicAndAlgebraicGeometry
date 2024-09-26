use super::{AttachContext, AttachToNet, ProofLink};
use crate::{
    errors::Error,
    proof_structure::{ProofStructure, VertexLabel},
};
use mll::formula::Formula;
use std::rc::Rc;

pub struct TensorLink {
    premise_left: Formula,
    premise_right: Formula,
}

impl TensorLink {
    pub fn new(premise_left: Formula, premise_right: Formula) -> TensorLink {
        TensorLink {
            premise_left,
            premise_right,
        }
    }
}

impl From<TensorLink> for ProofLink {
    fn from(tensor: TensorLink) -> ProofLink {
        ProofLink::TensorLink(tensor)
    }
}

impl AttachToNet for TensorLink {
    fn attach(self, net: &mut ProofStructure, ctx: AttachContext) -> Result<(), Error> {
        let left = ctx.prev_left.ok_or(Error::MissingPremise)?;
        let right = ctx.prev_right.ok_or(Error::MissingPremise)?;
        let next = ctx.next_left.ok_or(Error::MissingConclusion)?;
        let new_vert = net.add_vertex(VertexLabel::Tensor);
        net.add_edge(left, &new_vert, self.premise_left.clone())?;
        net.add_edge(right, &new_vert, self.premise_right.clone())?;
        net.add_edge(
            &new_vert,
            next,
            Formula::Tensor(Rc::new(self.premise_left), Rc::new(self.premise_right)),
        )?;
        Ok(())
    }
}
