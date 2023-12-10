use std::cmp::Ordering;

use crate::direction::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn go(self, dir: Direction) -> Option<Position> {
        let Position(i, j) = self;
        match dir {
            Direction::North => i.checked_sub(1).map(|i| Position(i, j)),
            Direction::East => Some(Position(i, j + 1)),
            Direction::South => Some(Position(i + 1, j)),
            Direction::West => j.checked_sub(1).map(|j| Position(i, j)),
        }
    }

    pub fn dir(self, to: Position) -> Direction {
        match (to.0.cmp(&self.0), to.1.cmp(&self.1)) {
            (Ordering::Less, Ordering::Equal) => Direction::North,
            (Ordering::Equal, Ordering::Greater) => Direction::East,
            (Ordering::Greater, Ordering::Equal) => Direction::South,
            (Ordering::Equal, Ordering::Less) => Direction::West,
            _ => panic!("expected orthogonally adjacent positions"),
        }
    }
}
