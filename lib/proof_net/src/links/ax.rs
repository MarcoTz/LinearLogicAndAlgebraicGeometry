use super::{AttachContext, AttachToNet, ProofLink};
use crate::{
    errors::Error,
    proof_structure::{ProofStructure, VertexLabel},
};
use mll::formula::Formula;
use std::ops::Neg;

pub struct AxLink {
    active: Formula,
}

impl AxLink {
    pub fn new(active: Formula) -> AxLink {
        AxLink { active }
    }
}

impl From<AxLink> for ProofLink {
    fn from(ax: AxLink) -> ProofLink {
        ProofLink::AxLink(ax)
    }
}

impl From<Formula> for AxLink {
    fn from(active: Formula) -> AxLink {
        AxLink::new(active)
    }
}

impl AttachToNet for AxLink {
    fn attach(self, net: &mut ProofStructure, ctx: AttachContext) -> Result<(), Error> {
        let next_left = match ctx.next_left {
            Some(l) => l,
            None => &net.add_vertex(VertexLabel::C),
        };
        let next_right = match ctx.next_right {
            Some(r) => r,
            None => &net.add_vertex(VertexLabel::C),
        };
        let new_vert = net.add_vertex(VertexLabel::Ax);
        net.add_edge(&new_vert, next_left, self.active.clone())?;
        net.add_edge(&new_vert, next_right, self.active.neg())?;
        Ok(())
    }
}
