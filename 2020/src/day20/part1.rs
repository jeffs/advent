#![allow(dead_code)]
use super::abutment::Abutment;
use super::projection::Projection;
use super::tile::Tile;
use crate::error::NoSolution;
use std::collections::HashSet;
use std::error::Error;

fn is_valid(abutments: &[Abutment]) -> bool {
    match abutments.len() {
        n @ (2..=4) => {
            let unique: HashSet<_> = abutments.iter().collect();
            unique.len() == n
        }
        _ => false,
    }
}

fn find_corners_imp(projections: &mut Vec<Projection>, tiles: &[Tile]) -> Option<[u64; 4]> {
    if tiles.is_empty() {
        for p in projections.iter() {
            let abutments: Vec<_> = projections.iter().flat_map(|q| p.abuts(q)).collect();
            if !is_valid(&abutments) {
                return None;
            }
        }
        let corner_ids: Vec<_> = projections
            .iter()
            .filter(|projection| projection.is_corner(projections))
            .map(|projection| projection.tile_id)
            .collect();
        match corner_ids.as_slice() {
            &[a, b, c, d] => Some([a, b, c, d]),
            _ => None,
        }
    } else {
        let tile = &tiles[0];
        for projection in &tile.projections() {
            projections.push(projection.clone());
            let corner_ids = find_corners_imp(projections, &tiles[1..]);
            if corner_ids.is_some() {
                return corner_ids;
            }
            projections.pop();
        }
        None
    }
}

/// Returns corner tile IDs, or None if four corners can't be found.
fn find_corners(tiles: &[Tile]) -> Option<[u64; 4]> {
    find_corners_imp(&mut Vec::new(), tiles)
}

pub fn solve(text: &str) -> Result<u64, Box<dyn Error>> {
    let mut tiles = Vec::new();
    for paragraph in text.trim().split("\n\n") {
        tiles.push(paragraph.parse()?);
    }
    let corner_ids = find_corners(&tiles).ok_or_else(|| Box::new(NoSolution))?;
    println!("{:?}", corner_ids);
    Ok(corner_ids.iter().product())
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
