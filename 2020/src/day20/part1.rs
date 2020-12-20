#![allow(dead_code)]
use super::projection::Projection;
use super::tile::Tile;
use crate::error::NoSolution;
use std::error::Error;

//  given mutable references to sets of projected and raw tiles:
//      if the set of raw tiles is empty
//          find the set of corners from the projections
//          if we found exactly four corners
//              return Some(corner IDs)
//          else
//              return None
//      else
//          remove a tile from the raw set
//          for each projection of the tile
//              add the projection to the projected set
//              let corner_ids = recurse(projections, tiles);
//              if corner_ids.is_some()
//                  return corner_ids
//              remove the projection from the projected set
//          return the tile to the raw set
//          return None
fn find_corners_imp(projections: &mut Vec<Projection>, tiles: & [Tile]) -> Option<[u64; 4]> {
    if tiles.is_empty() {
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
    for paragraph in text.split("\n\n") {
        tiles.push(paragraph.parse()?);
    }
    match find_corners(&tiles) {
        Some(corner_ids) => Ok(corner_ids.iter().product()),
        None => Err(Box::new(NoSolution)),
    }
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
