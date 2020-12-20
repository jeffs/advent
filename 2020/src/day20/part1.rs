#![allow(dead_code, unused_imports, unused_variables)]
use super::abutment::Abutment;
use super::projection::Projection;
use super::tile::Tile;
use crate::error::NoSolution;
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug)]
struct NeighborSet<'a> {
    rights: Vec<&'a Projection>,
    downs: Vec<&'a Projection>,
}

fn collect_projections(tiles: &[Tile]) -> Vec<Projection> {
    let mut projections = Vec::new();
    for tile in tiles {
        projections.extend(tile.projections().iter().cloned());
    }
    projections
}

fn graph_neighbors(projections: &[Projection]) -> HashMap<&Projection, NeighborSet> {
    let mut neighbors = HashMap::new();
    for p in projections {
        let rights = projections.iter().filter(|q| p.right == q.left).collect();
        let downs = projections.iter().filter(|q| p.bottom == q.top).collect();
        neighbors.insert(p, NeighborSet { rights, downs });
    }
    neighbors
}

pub fn solve(text: &str) -> Result<u64, Box<dyn Error>> {
    //  map each projection to its possible neighbors
    //      - for each other tile, consider all its projections
    //  pick a possible top left, and start trying to fill the grid
    //      - take care not to place multiple projections of any tile
    let tiles = Tile::parse_all(text)?;
    let projections = collect_projections(&tiles);
    let neighbors = graph_neighbors(&projections);
    println!("{:#?}", neighbors);
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let input_path = "tests/day20/sample1";
        let text = fs::read_to_string(input_path).unwrap();
        assert_eq!(20899048083289, solve(&text).unwrap());
    }
}
