use crate::direction::Direction;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    pub fn from_ascii(c: u8) -> Tile {
        match c {
            b'|' => Tile::Vertical,
            b'-' => Tile::Horizontal,
            b'L' => Tile::NorthEast,
            b'J' => Tile::NorthWest,
            b'7' => Tile::SouthWest,
            b'F' => Tile::SouthEast,
            b'.' => Tile::Ground,
            b'S' => Tile::Start,
            _ => panic!("{}: bad tile", c as char),
        }
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
        assert!(Tile::Vertical.is_open_to(Direction::North));
        assert!(!Tile::Vertical.is_open_to(Direction::East));
        assert!(Tile::Vertical.is_open_to(Direction::South));
        assert!(!Tile::Vertical.is_open_to(Direction::West));

        assert!(!Tile::Horizontal.is_open_to(Direction::North));
        assert!(Tile::Horizontal.is_open_to(Direction::East));
        assert!(!Tile::Horizontal.is_open_to(Direction::South));
        assert!(Tile::Horizontal.is_open_to(Direction::West));

        assert!(Tile::NorthEast.is_open_to(Direction::North));
        assert!(Tile::NorthEast.is_open_to(Direction::East));
        assert!(!Tile::NorthEast.is_open_to(Direction::South));
        assert!(!Tile::NorthEast.is_open_to(Direction::West));

        assert!(Tile::NorthWest.is_open_to(Direction::North));
        assert!(!Tile::NorthWest.is_open_to(Direction::East));
        assert!(!Tile::NorthWest.is_open_to(Direction::South));
        assert!(Tile::NorthWest.is_open_to(Direction::West));

        assert!(!Tile::SouthWest.is_open_to(Direction::North));
        assert!(!Tile::SouthWest.is_open_to(Direction::East));
        assert!(Tile::SouthWest.is_open_to(Direction::South));
        assert!(Tile::SouthWest.is_open_to(Direction::West));

        assert!(!Tile::SouthEast.is_open_to(Direction::North));
        assert!(Tile::SouthEast.is_open_to(Direction::East));
        assert!(Tile::SouthEast.is_open_to(Direction::South));
        assert!(!Tile::SouthEast.is_open_to(Direction::West));

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
