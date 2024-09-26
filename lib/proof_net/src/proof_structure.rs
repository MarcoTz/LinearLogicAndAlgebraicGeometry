use super::errors::Error;
use mll::formula::Formula;
use std::{collections::HashSet, fmt, ops::Neg};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VertexLabel {
    Ax,
    Cut,
    Tensor,
    Par,
    Bang,
    Quest,
    C,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Vertex {
    label: VertexLabel,
    num: i32,
}

pub struct Edge {
    pub from: Vertex,
    pub to: Vertex,
    pub label: Formula,
}

pub struct ProofStructure {
    vertices: HashSet<Vertex>,
    edges: Vec<Edge>,
}

impl Default for ProofStructure {
    fn default() -> ProofStructure {
        ProofStructure::new()
    }
}

impl ProofStructure {
    pub fn new() -> ProofStructure {
        ProofStructure {
            vertices: HashSet::new(),
            edges: vec![],
        }
    }

    fn new_vert(&mut self, label: VertexLabel) -> Vertex {
        let new_num = self.vertices.len() as i32;
        let new_vert = Vertex {
            label,
            num: new_num,
        };
        self.vertices.insert(new_vert.clone());
        new_vert
    }

    fn find_vert(&self, vert: &Vertex) -> Result<&Vertex, Error> {
        self.vertices
            .iter()
            .find(|v| **v == *vert)
            .ok_or(Error::VertexNotFound(vert.to_owned()))
    }

    pub fn add_ax_link(
        &mut self,
        conclusion: Formula,
        next_pos: &Vertex,
        next_neg: &Vertex,
    ) -> Result<(), Error> {
        let new_vert = self.new_vert(VertexLabel::Ax);
        let to_pos = self.find_vert(next_pos)?;
        let to_neg = self.find_vert(next_neg)?;
        let edg_left = Edge {
            from: new_vert.clone(),
            to: to_pos.to_owned(),
            label: conclusion.clone(),
        };
        let edg_right = Edge {
            from: new_vert.clone(),
            to: to_neg.to_owned(),
            label: conclusion.neg(),
        };
        self.edges.push(edg_left);
        self.edges.push(edg_right);
        Ok(())
    }

    pub fn add_cut_link(
        &mut self,
        premise: Formula,
        last_pos: &Vertex,
        last_neg: &Vertex,
    ) -> Result<(), Error> {
        let new_vert = self.new_vert(VertexLabel::Cut);
        let last_pos = self.find_vert(last_pos)?;
        let last_neg = self.find_vert(last_neg)?;
        let edg_left = Edge {
            to: new_vert.clone(),
            from: last_pos.to_owned(),
            label: premise.clone(),
        };
        let edg_right = Edge {
            to: new_vert,
            from: last_neg.to_owned(),
            label: premise.neg(),
        };
        self.edges.push(edg_left);
        self.edges.push(edg_right);
        Ok(())
    }

    pub fn add_tensor_link(
        &mut self,
        prem_left: Formula,
        prem_right: Formula,
        last_left: &Vertex,
        last_right: &Vertex,
    ) -> Result<(), Error> {
        let new_vert = self.new_vert(VertexLabel::Tensor);
        let last_left = self.find_vert(last_left)?;
        let last_right = self.find_vert(last_right)?;
        let edg_left = Edge {
            from: last_left.to_owned(),
            to: new_vert.clone(),
            label: prem_left,
        };
        let edg_right = Edge {
            from: last_right.to_owned(),
            to: new_vert,
            label: prem_right,
        };
        self.edges.push(edg_left);
        self.edges.push(edg_right);
        Ok(())
    }

    pub fn add_par_link(
        &mut self,
        prem_left: Formula,
        prem_right: Formula,
        last_left: &Vertex,
        last_right: &Vertex,
    ) -> Result<(), Error> {
        let new_vert = self.new_vert(VertexLabel::Par);
        let last_left = self.find_vert(last_left)?;
        let last_right = self.find_vert(last_right)?;
        let edg_left = Edge {
            from: last_left.to_owned(),
            to: new_vert.clone(),
            label: prem_left,
        };
        let edg_right = Edge {
            from: last_right.to_owned(),
            to: new_vert.clone(),
            label: prem_right,
        };
        self.edges.push(edg_left);
        self.edges.push(edg_right);
        Ok(())
    }

    pub fn add_conclusion(&mut self, premise: Formula, last: &Vertex) -> Result<(), Error> {
        let new_vert = self.new_vert(VertexLabel::C);
        let last = self.find_vert(last)?;
        let new_edg = Edge {
            to: new_vert,
            from: last.to_owned(),
            label: premise,
        };
        self.edges.push(new_edg);
        Ok(())
    }

    pub fn occurs(&self, f: &Formula) -> bool {
        self.edges.iter().any(|edg| edg.label == *f)
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.label.fmt(f)
    }
}

impl fmt::Display for VertexLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            VertexLabel::Ax => f.write_str("Ax"),
            VertexLabel::Cut => f.write_str("Cut"),
            VertexLabel::Tensor => f.write_str("Tensor"),
            VertexLabel::Par => f.write_str("Par"),
            VertexLabel::Bang => f.write_str("!"),
            VertexLabel::Quest => f.write_str("?"),
            VertexLabel::C => f.write_str("c"),
        }
    }
}
