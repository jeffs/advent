use super::tile::Projection;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NeighborSet<'a> {
    pub rights: Vec<&'a Projection>,
    pub downs: Vec<&'a Projection>,
}

impl NeighborSet<'_> {
    pub fn graph(projections: &[Projection]) -> HashMap<&Projection, NeighborSet> {
        let mut neighbors = HashMap::new();
        for p in projections {
            let rights = projections.iter().filter(|q| p.right == q.left).collect();
            let downs = projections.iter().filter(|q| p.bottom == q.top).collect();
            neighbors.insert(p, NeighborSet { rights, downs });
        }
        neighbors
    }
}
