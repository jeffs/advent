use crate::direction::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    Start,
}

impl Tile {
    pub fn from_ascii(c: u8) -> Tile {
        match c {
            b'|' => Tile::VerticalPipe,
            b'-' => Tile::HorizontalPipe,
            b'L' => Tile::NorthEastBend,
            b'J' => Tile::NorthWestBend,
            b'7' => Tile::SouthWestBend,
            b'F' => Tile::SouthEastBend,
            b'.' | b'I' | b'O' => Tile::Ground,
            b'S' => Tile::Start,
            _ => panic!("{}: bad tile", c as char),
        }
    }

    pub fn from_exits(exits: impl IntoIterator<Item = Direction>) -> Tile {
        let mut exits = exits.into_iter();
        let mut dirs = [
            exits.next().expect("an exit"),
            exits.next().expect("another exit"),
        ];
        if let Some(exit) = exits.next() {
            panic!("{exit:?}: unexpected third exit")
        }
        dirs.sort();
        match dirs {
            [dir1, dir2] if dir1 == dir2 => panic!("exits should differ"),
            [Direction::North, Direction::East] => Tile::NorthEastBend,
            [Direction::North, Direction::South] => Tile::VerticalPipe,
            [Direction::North, Direction::West] => Tile::NorthWestBend,
            [Direction::East, Direction::South] => Tile::SouthEastBend,
            [Direction::East, Direction::West] => Tile::HorizontalPipe,
            [Direction::South, Direction::West] => Tile::SouthWestBend,
            _ => unreachable!(),
        }
    }

    pub fn to_ascii(self) -> u8 {
        match self {
            Tile::VerticalPipe => b'|',
            Tile::HorizontalPipe => b'-',
            Tile::NorthEastBend => b'L',
            Tile::NorthWestBend => b'J',
            Tile::SouthWestBend => b'7',
            Tile::SouthEastBend => b'F',
            Tile::Ground => b'.',
            Tile::Start => b'S',
        }
    }

    pub fn expand_east(self) -> Tile {
        match self {
            Tile::VerticalPipe => Tile::Ground,
            Tile::HorizontalPipe => Tile::HorizontalPipe,
            Tile::NorthEastBend => Tile::HorizontalPipe,
            Tile::NorthWestBend => Tile::Ground,
            Tile::SouthWestBend => Tile::Ground,
            Tile::SouthEastBend => Tile::HorizontalPipe,
            Tile::Ground => Tile::Ground,
            Tile::Start => panic!("can't expand start tile"),
        }
    }

    pub fn expand_south(self) -> Tile {
        match self {
            Tile::VerticalPipe => Tile::VerticalPipe,
            Tile::HorizontalPipe => Tile::Ground,
            Tile::NorthEastBend => Tile::Ground,
            Tile::NorthWestBend => Tile::Ground,
            Tile::SouthWestBend => Tile::VerticalPipe,
            Tile::SouthEastBend => Tile::VerticalPipe,
            Tile::Ground => Tile::Ground,
            Tile::Start => panic!("can't expand start tile"),
        }
    }

    pub fn is_ground(self) -> bool {
        self == Tile::Ground
    }

    pub fn is_open_to(self, dir: Direction) -> bool {
        0xF06C93A5u32 & 1 << (self as u32 * 4 + dir as u32) != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_is_open_to() {
        assert!(Tile::VerticalPipe.is_open_to(Direction::North));
        assert!(!Tile::VerticalPipe.is_open_to(Direction::East));
        assert!(Tile::VerticalPipe.is_open_to(Direction::South));
        assert!(!Tile::VerticalPipe.is_open_to(Direction::West));

        assert!(!Tile::HorizontalPipe.is_open_to(Direction::North));
        assert!(Tile::HorizontalPipe.is_open_to(Direction::East));
        assert!(!Tile::HorizontalPipe.is_open_to(Direction::South));
        assert!(Tile::HorizontalPipe.is_open_to(Direction::West));

        assert!(Tile::NorthEastBend.is_open_to(Direction::North));
        assert!(Tile::NorthEastBend.is_open_to(Direction::East));
        assert!(!Tile::NorthEastBend.is_open_to(Direction::South));
        assert!(!Tile::NorthEastBend.is_open_to(Direction::West));

        assert!(Tile::NorthWestBend.is_open_to(Direction::North));
        assert!(!Tile::NorthWestBend.is_open_to(Direction::East));
        assert!(!Tile::NorthWestBend.is_open_to(Direction::South));
        assert!(Tile::NorthWestBend.is_open_to(Direction::West));

        assert!(!Tile::SouthWestBend.is_open_to(Direction::North));
        assert!(!Tile::SouthWestBend.is_open_to(Direction::East));
        assert!(Tile::SouthWestBend.is_open_to(Direction::South));
        assert!(Tile::SouthWestBend.is_open_to(Direction::West));

        assert!(!Tile::SouthEastBend.is_open_to(Direction::North));
        assert!(Tile::SouthEastBend.is_open_to(Direction::East));
        assert!(Tile::SouthEastBend.is_open_to(Direction::South));
        assert!(!Tile::SouthEastBend.is_open_to(Direction::West));

        assert!(!Tile::Ground.is_open_to(Direction::North));
        assert!(!Tile::Ground.is_open_to(Direction::East));
        assert!(!Tile::Ground.is_open_to(Direction::South));
        assert!(!Tile::Ground.is_open_to(Direction::West));

        assert!(Tile::Start.is_open_to(Direction::North));
        assert!(Tile::Start.is_open_to(Direction::East));
        assert!(Tile::Start.is_open_to(Direction::South));
        assert!(Tile::Start.is_open_to(Direction::West));
    }
}
