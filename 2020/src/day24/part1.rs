//! We'll model our hex grid as a square grid by mapping two of the
//! semicardinal directions to north and south.  There's a little wackiness
//! though, because hex grid axes aren't orthogonal like square grids are:
//! We'll have to alternate which pair of semicardinals to remap (either
//! westerly or easterly) from one row to the next.  Assuming the reference
//! tile is at the origin (0, 0), we'll map NE/SE to north and south in even
//! numbered rows, but map NW/SW in odd ones.
//!
//! Note that unmapped semicardinals mean the same thing they would in any
//! rectangular grid.  For example, going SW from a tile in an even numbered
//! row means going diagonally south and west.

use std::collections::HashSet;
use std::ops::AddAssign;

enum HexDirection {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
}

enum SquareDirection {
    East,
    NorthEast,
    North,
    NorthWest,
    West,
    SouthWest,
    South,
    SouthEast,
}

struct HexDirections<'a> {
    line: &'a str,
}

impl Iterator for HexDirections<'_> {
    type Item = HexDirection;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.starts_with('e') {
            self.line = &self.line[1..];
            Some(HexDirection::East)
        } else if self.line.starts_with("ne") {
            self.line = &self.line[2..];
            Some(HexDirection::NorthEast)
        } else if self.line.starts_with("nw") {
            self.line = &self.line[2..];
            Some(HexDirection::NorthWest)
        } else if self.line.starts_with('w') {
            self.line = &self.line[1..];
            Some(HexDirection::West)
        } else if self.line.starts_with("sw") {
            self.line = &self.line[2..];
            Some(HexDirection::SouthWest)
        } else if self.line.starts_with("se") {
            self.line = &self.line[2..];
            Some(HexDirection::SouthEast)
        } else {
            None
        }
    }
}

fn normalize(hex: HexDirection, latitude: isize) -> SquareDirection {
    let even = latitude % 2 == 0;
    match hex {
        HexDirection::NorthEast if even => SquareDirection::North,
        HexDirection::SouthEast if even => SquareDirection::South,
        HexDirection::NorthWest if !even => SquareDirection::North,
        HexDirection::SouthWest if !even => SquareDirection::South,
        HexDirection::East => SquareDirection::East,
        HexDirection::NorthEast => SquareDirection::NorthEast,
        HexDirection::NorthWest => SquareDirection::NorthWest,
        HexDirection::West => SquareDirection::West,
        HexDirection::SouthWest => SquareDirection::SouthWest,
        HexDirection::SouthEast => SquareDirection::SouthEast,
    }
}

/// Latitude and longitude.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct LatLon(isize, isize);

impl LatLon {
    fn hence(mut self, hexes: HexDirections) -> LatLon {
        for hex in hexes {
            self += normalize(hex, self.0);
        }
        self
    }
}

impl AddAssign<SquareDirection> for LatLon {
    fn add_assign(&mut self, other: SquareDirection) {
        let (dy, dx) = match other {
            SquareDirection::East => (0, 1),
            SquareDirection::NorthEast => (1, 1),
            SquareDirection::North => (1, 0),
            SquareDirection::NorthWest => (1, -1),
            SquareDirection::West => (0, -1),
            SquareDirection::SouthWest => (-1, -1),
            SquareDirection::South => (-1, 0),
            SquareDirection::SouthEast => (-1, 1),
        };
        self.0 += dy;
        self.1 += dx;
    }
}

pub fn solve(text: &str) -> usize {
    let mut black: HashSet<LatLon> = HashSet::new();
    let origin = LatLon(0, 0);
    for line in text.lines() {
        let tile = origin.hence(HexDirections { line });
        if black.contains(&tile) {
            black.remove(&tile);
        } else {
            black.insert(tile);
        }
    }
    black.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let text = fs::read_to_string("tests/day24/sample1").unwrap();
        assert_eq!(10, solve(&text));
    }
}
