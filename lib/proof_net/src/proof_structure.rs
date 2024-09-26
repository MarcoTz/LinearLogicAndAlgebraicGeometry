use super::errors::Error;
use mll::formula::Formula;
use std::{collections::HashSet, fmt};

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

#[derive(Clone)]
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

    pub fn add_vertex(&mut self, label: VertexLabel) -> Vertex {
        let max_ind = self
            .vertices
            .iter()
            .max_by(|v1, v2| v1.num.cmp(&v2.num))
            .map(|v| v.num)
            .unwrap_or_default();
        let new_vert = Vertex {
            label,
            num: max_ind + 1,
        };
        self.vertices.insert(new_vert.clone());
        new_vert
    }

    fn find_vertex(&self, vert: &Vertex) -> Result<Vertex, Error> {
        self.vertices
            .iter()
            .find(|v| **v == *vert)
            .cloned()
            .ok_or(Error::VertexNotFound(vert.to_owned()))
    }

    pub fn add_edge(&mut self, from: &Vertex, to: &Vertex, label: Formula) -> Result<Edge, Error> {
        let from = self.find_vertex(from)?;
        let to = self.find_vertex(to)?;
        let edge = Edge { from, to, label };
        self.edges.push(edge.clone());
        Ok(edge)
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
