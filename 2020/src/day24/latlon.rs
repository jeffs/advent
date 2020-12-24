use super::direction::{HexDirection, HexDirections, SquareDirection};
use std::ops::AddAssign;

/// Latitude and longitude.
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct LatLon(pub isize, pub isize);

impl LatLon {
    pub fn hence(mut self, hexes: HexDirections) -> LatLon {
        for hex in hexes {
            self += self.normalize(hex);
        }
        self
    }

    fn normalize(&self, hex: HexDirection) -> SquareDirection {
        let even = self.0 % 2 == 0;
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
