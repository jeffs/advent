use super::{CardinalDirection, Point, RelativeDirection};

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

    pub fn north(self, distance: usize) -> Ship {
        Ship {
            dir: self.dir,
            pos: self.pos.north(distance),
        }
    }

    pub fn east(self, distance: usize) -> Ship {
        Ship {
            dir: self.dir,
            pos: self.pos.east(distance),
        }
    }

    pub fn south(self, distance: usize) -> Ship {
        Ship {
            dir: self.dir,
            pos: self.pos.south(distance),
        }
    }

    pub fn west(self, distance: usize) -> Ship {
        Ship {
            dir: self.dir,
            pos: self.pos.west(distance),
        }
    }

    pub fn left(self, degrees: usize) -> Ship {
        Ship {
            dir: self.dir.turn(RelativeDirection::Left, degrees),
            pos: self.pos,
        }
    }

    pub fn right(self, degrees: usize) -> Ship {
        Ship {
            dir: self.dir.turn(RelativeDirection::Right, degrees),
            pos: self.pos,
        }
    }

    pub fn forward(self, distance: usize) -> Ship {
        match self.dir {
            CardinalDirection::North => self.north(distance),
            CardinalDirection::South => self.south(distance),
            CardinalDirection::East => self.east(distance),
            CardinalDirection::West => self.west(distance),
        }
    }

    /// Returns this Ship's Manhattan distance from the origin.
    pub fn distance(self) -> usize {
        self.pos.x.abs() as usize + self.pos.y.abs() as usize
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn toward(self, pos: Point, distance: usize) -> Ship {
        let dx = distance * (pos.x - self.pos.x).abs() as usize;
        let dy = distance * (pos.y - self.pos.y).abs() as usize;
        println!("dx={}, dy={}", dx, dy);
        match (pos.x < self.pos.x, pos.y < self.pos.y) {
            (false, false) => self.east(dx).north(dy),
            (false, true) => self.east(dx).south(dy),
            (true, false) => self.west(dx).north(dy),
            (true, true) => self.west(dx).south(dy),
        }
    }
}
