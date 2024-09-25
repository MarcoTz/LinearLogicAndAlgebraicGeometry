use std::fmt;

pub trait DirectedMultiGraph {
    type VertexLabel;
    type EdgeLabel: Clone + fmt::Display;
    type Error: std::error::Error;

    fn edges(&self) -> Vec<Self::EdgeLabel>;
    fn vertices(&self) -> Vec<Self::VertexLabel>;
    fn incidence(
        &self,
        label: Self::EdgeLabel,
    ) -> Result<(Self::VertexLabel, Self::VertexLabel), Self::Error>;
}
