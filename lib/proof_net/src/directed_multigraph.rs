pub trait GraphVertex {
    type Label;
    fn get_label(&self) -> Self::Label;
}

pub trait GraphEdge {
    type Label;
    type Vertex: GraphVertex;

    fn get_label(&self) -> Self::Label;
    fn from(&self) -> Self::Vertex;
    fn to(&self) -> Self::Vertex;
}

pub trait DirectedMultiGraph: Default {
    type Vertex: GraphVertex;
    type Edge: GraphEdge<Vertex = Self::Vertex>;
    type Error: std::error::Error;

    fn get_vertices(&self) -> Vec<Self::Vertex>;
    fn add_vertex(
        &mut self,
        label: <Self::Vertex as GraphVertex>::Label,
    ) -> Result<Self::Vertex, Self::Error>;
    fn find_vertex(&self, label: &<Self::Vertex as GraphVertex>::Label) -> Option<Self::Vertex>;

    fn get_edges(&self) -> Vec<Self::Edge>;
    fn add_edge(
        &mut self,
        from: &<Self::Vertex as GraphVertex>::Label,
        to: &<Self::Vertex as GraphVertex>::Label,
        label: <Self::Edge as GraphEdge>::Label,
    ) -> Result<Self::Edge, Self::Error>;
    fn find_edge(&self, label: &<Self::Edge as GraphEdge>::Label) -> Option<Self::Edge>;

    fn disjoint_union(&mut self, other: Self) -> Result<(), Self::Error> {
        let other_verts = other.get_vertices();
        for other_vert in other_verts {
            let other_label = other_vert.get_label();
            self.add_vertex(other_label)?;
        }
        let other_edges = other.get_edges();
        for other_edge in other_edges {
            let vert_from = other_edge.from();
            let vert_to = other_edge.to();
            self.add_edge(
                &vert_from.get_label(),
                &vert_to.get_label(),
                other_edge.get_label(),
            )?;
        }
        Ok(())
    }
}
