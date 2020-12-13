use super::vector;
use super::{CardinalDirection, Point, Vector};

#[derive(Debug)]
pub struct Ship {
    dir: CardinalDirection,
    pos: Point,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            dir: CardinalDirection::East,
            pos: Point { x: 0, y: 0 },
        }
    }

    pub fn strafe(&self, way: Vector) -> Ship {
        Ship {
            dir: self.dir,
            pos: self.pos + way,
        }
    }

    pub fn turn(&self, degrees: isize) -> Ship {
        Ship {
            dir: self.dir.turn(degrees),
            pos: self.pos,
        }
    }

    pub fn forward(&self, distance: usize) -> Ship {
        let unit = match self.dir {
            CardinalDirection::North => vector::NORTH,
            CardinalDirection::South => vector::SOUTH,
            CardinalDirection::East => vector::EAST,
            CardinalDirection::West => vector::WEST,
        };
        self.strafe(unit * distance as isize)
    }

    /// Returns this Ship's Manhattan distance from the origin.
    pub fn distance(&self) -> usize {
        self.pos.x.abs() as usize + self.pos.y.abs() as usize
    }

    pub fn wayward(&self, way: Vector, distance: usize) -> Ship {
        Ship {
            dir: self.dir,
            pos: self.pos + way * distance as isize,
        }
    }
}
