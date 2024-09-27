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
            .map(|v| v.num)
            .max()
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

    pub fn disjoint_union(&mut self, other: ProofStructure) {
        let max_ind = self
            .vertices
            .iter()
            .map(|v| v.num)
            .max()
            .unwrap_or_default()
            + 1;
        let new_verts_other: Vec<Vertex> = other
            .vertices
            .iter()
            .map(|v| Vertex {
                label: v.label.clone(),
                num: v.num + max_ind,
            })
            .collect();
        self.vertices.extend(new_verts_other);
        let new_edges_other: Vec<Edge> = other
            .edges
            .iter()
            .map(|e| Edge {
                from: Vertex {
                    label: e.from.label.clone(),
                    num: e.from.num + max_ind,
                },
                to: Vertex {
                    label: e.to.label.clone(),
                    num: e.to.num + max_ind,
                },
                label: e.label.clone(),
            })
            .collect();
        self.edges.extend(new_edges_other);
    }

    pub fn find_conclusion(&self, conc: &Formula) -> Option<Vertex> {
        let possible_edges: Vec<&Edge> =
            self.edges.iter().filter(|edg| edg.label == *conc).collect();
        let possible_vertices: Vec<Vertex> =
            possible_edges.iter().map(|edg| edg.to.clone()).collect();
        possible_vertices
            .into_iter()
            .filter(|v| v.label == VertexLabel::C)
            .collect::<Vec<Vertex>>()
            .first()
            .cloned()
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
