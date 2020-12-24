use super::direction::{HexDirection, HexDirections, SquareDirection};
use std::ops::{Add, AddAssign};

pub struct Neighbors {
    center: LatLon,
    directions: HexDirections,
}

impl Neighbors {
    fn new(center: LatLon) -> Neighbors {
        Neighbors {
            center,
            directions: HexDirection::all(),
        }
    }
}

impl Iterator for Neighbors {
    type Item = LatLon;

    fn next(&mut self) -> Option<Self::Item> {
        self.directions
            .next()
            .map(|direction| self.center + direction)
    }
}

/// Latitude and longitude.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct LatLon(pub isize, pub isize);

impl LatLon {
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

    pub fn hence<I>(self, hexes: I) -> LatLon
    where
        I: Iterator<Item = HexDirection>,
    {
        hexes.fold(self, |latlon, hex| latlon + hex)
    }

    pub fn neighbors(self) -> Neighbors {
        Neighbors::new(self)
    }
}

impl Add<HexDirection> for LatLon {
    type Output = Self;

    fn add(self, other: HexDirection) -> Self::Output {
        self + self.normalize(other)
    }
}

impl AddAssign<HexDirection> for LatLon {
    fn add_assign(&mut self, other: HexDirection) {
        *self = *self + other;
    }
}

impl Add<SquareDirection> for LatLon {
    type Output = Self;

    fn add(self, other: SquareDirection) -> Self::Output {
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
        LatLon(self.0 + dy, self.1 + dx)
    }
}

impl AddAssign<SquareDirection> for LatLon {
    fn add_assign(&mut self, other: SquareDirection) {
        *self = *self + other;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn neighbors_even() {
        let neighbors: Vec<_> = LatLon(0, 0).neighbors().collect();
        let want = [
            LatLon(0, 1),
            LatLon(1, 0),
            LatLon(1, -1), 
            LatLon(0, -1), 
            LatLon(-1, -1), 
            LatLon(-1, 0),
        ];
        assert_eq!(want.to_vec(), neighbors);
    }

}
