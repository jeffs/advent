use crate::direction::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position(pub usize, pub usize);

impl Position {
    pub fn go(&self, dir: Direction) -> Option<Position> {
        let Position(i, j) = *self;
        match dir {
            Direction::North => i.checked_sub(1).map(|i| Position(i, j)),
            Direction::East => Some(Position(i, j + 1)),
            Direction::South => Some(Position(i + 1, j)),
            Direction::West => j.checked_sub(1).map(|j| Position(i, j)),
        }
    }
}
