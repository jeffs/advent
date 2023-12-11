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
            b'.' => Tile::Ground,
            b'S' => Tile::Start,
            _ => panic!("{}: bad tile", c as char),
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
