use super::{
    directed_multigraph::{DirectedMultiGraph, GraphEdge, GraphVertex},
    errors::Error,
};
use mll::formula::Formula;
use std::{collections::HashSet, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleLabel {
    Ax,
    Cut,
    Tensor,
    Par,
    Bang,
    Quest,
    C,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct VertexLabel {
    pub rule: RuleLabel,
    _num: i32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Vertex {
    pub label: VertexLabel,
}

impl GraphVertex for Vertex {
    type Label = VertexLabel;
    fn get_label(&self) -> Self::Label {
        self.label.to_owned()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Edge {
    pub from: Vertex,
    pub to: Vertex,
    pub label: Formula,
}

impl GraphEdge for Edge {
    type Label = Formula;
    type Vertex = Vertex;
    fn get_label(&self) -> Self::Label {
        self.label.to_owned()
    }
    fn from(&self) -> Self::Vertex {
        self.from.to_owned()
    }
    fn to(&self) -> Self::Vertex {
        self.to.to_owned()
    }
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

impl DirectedMultiGraph for ProofStructure {
    type Vertex = Vertex;
    type Edge = Edge;
    type Error = Error;

    fn get_vertices(&self) -> Vec<Self::Vertex> {
        self.vertices.iter().cloned().collect()
    }

    fn add_vertex(&mut self, label: VertexLabel) -> Result<Vertex, Error> {
        if self.find_vertex(&label).is_some() {
            Err(Error::VertexAlreadyExists(label.clone()))
        } else {
            Ok(())
        }?;
        let new_vert = Vertex { label };
        self.vertices.insert(new_vert.clone());
        Ok(new_vert)
    }

    fn find_vertex(&self, vert: &VertexLabel) -> Option<Vertex> {
        self.vertices
            .iter()
            .find(|v| v.get_label() == *vert)
            .cloned()
    }

    fn remove_vertex(&mut self, label: &VertexLabel) -> Result<(), Error> {
        let vertex = self
            .find_vertex(label)
            .ok_or(Error::VertexNotFound(label.to_owned()))?;
        let incoming = self.get_incoming(label)?;
        incoming
            .iter()
            .map(|e| self.remove_edge(&e.get_label()))
            .collect::<Result<Vec<()>, Error>>()?;
        let outgoing = self.get_outgoing(label)?;
        outgoing
            .iter()
            .map(|e| self.remove_edge(&e.get_label()))
            .collect::<Result<Vec<()>, Error>>()?;
        self.vertices.remove(&vertex);
        Ok(())
    }

    fn get_outgoing(&self, label: &VertexLabel) -> Result<Vec<Edge>, Error> {
        let vert = self
            .find_vertex(label)
            .ok_or(Error::VertexNotFound(label.to_owned()))?;
        Ok(self
            .edges
            .iter()
            .filter(|e| e.from() == vert)
            .cloned()
            .collect())
    }
    fn get_incoming(&self, label: &VertexLabel) -> Result<Vec<Edge>, Error> {
        let vert = self
            .find_vertex(label)
            .ok_or(Error::VertexNotFound(label.to_owned()))?;
        Ok(self
            .edges
            .iter()
            .filter(|e| e.to() == vert)
            .cloned()
            .collect())
    }

    fn get_edges(&self) -> Vec<Edge> {
        self.edges.to_owned()
    }

    fn add_edge(
        &mut self,
        from: &VertexLabel,
        to: &VertexLabel,
        label: Formula,
    ) -> Result<Edge, Error> {
        let from = self
            .find_vertex(from)
            .ok_or(Error::VertexNotFound(from.to_owned()))?;
        let to = self
            .find_vertex(to)
            .ok_or(Error::VertexNotFound(to.to_owned()))?;
        let edge = Edge { from, to, label };
        self.edges.push(edge.clone());
        Ok(edge)
    }

    fn find_edge(&self, label: &Formula) -> Option<Edge> {
        self.edges.iter().find(|e| e.get_label() == *label).cloned()
    }

    fn remove_edge(&mut self, label: &Formula) -> Result<(), Error> {
        let edge = self
            .find_edge(label)
            .ok_or(Error::EdgeNotFound(label.to_owned()))?;
        let ind = self.edges.iter().position(|e| *e == edge).unwrap();
        self.edges.remove(ind);
        Ok(())
    }
}

impl ProofStructure {
    pub fn new() -> ProofStructure {
        ProofStructure {
            vertices: HashSet::new(),
            edges: vec![],
        }
    }

    pub fn fresh_label(&self, rule: RuleLabel) -> VertexLabel {
        let max_ind = self
            .vertices
            .iter()
            .map(|v| v.get_label()._num)
            .max()
            .unwrap_or_default()
            + 1;
        VertexLabel {
            rule,
            _num: max_ind,
        }
    }

    pub fn find_conclusion(&self, conc: &Formula) -> Option<Vertex> {
        let possible_edges: Vec<&Edge> =
            self.edges.iter().filter(|edg| edg.label == *conc).collect();
        let possible_vertices: Vec<Vertex> =
            possible_edges.iter().map(|edg| edg.to.clone()).collect();
        possible_vertices
            .into_iter()
            .filter(|v| v.get_label().rule == RuleLabel::C)
            .collect::<Vec<Vertex>>()
            .first()
            .cloned()
    }

    pub fn get_cuts(&self) -> Vec<Vertex> {
        self.vertices
            .iter()
            .filter(|v| v.get_label().rule == RuleLabel::Cut)
            .cloned()
            .collect()
    }
}

impl fmt::Display for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.label.fmt(f)
    }
}

impl fmt::Display for VertexLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.rule.fmt(f)
    }
}

impl fmt::Display for RuleLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuleLabel::Ax => f.write_str("Ax"),
            RuleLabel::Cut => f.write_str("Cut"),
            RuleLabel::Tensor => f.write_str("Tensor"),
            RuleLabel::Par => f.write_str("Par"),
            RuleLabel::Bang => f.write_str("!"),
            RuleLabel::Quest => f.write_str("?"),
            RuleLabel::C => f.write_str("c"),
        }
    }
}
