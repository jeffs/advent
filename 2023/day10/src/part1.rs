#![allow(dead_code, unused_variables)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn reverse(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Position(usize, usize);

impl Position {
    fn go(&self, dir: Direction) -> Option<Position> {
        let Position(i, j) = *self;
        match dir {
            Direction::North => i.checked_sub(1).map(|i| Position(i, j)),
            Direction::East => Some(Position(i, j + 1)),
            Direction::South => Some(Position(i + 1, j)),
            Direction::West => j.checked_sub(1).map(|j| Position(i, j)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
enum Tile {
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
    fn is_open_to(self, dir: Direction) -> bool {
        0xF06C93A5u32 & 1 << (self as u32 * 4 + dir as u32) != 0
    }
}

impl Tile {
    fn from_ascii(c: u8) -> Tile {
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
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn from_str(text: &str) -> Grid {
        Grid(
            text.lines()
                .map(|line| line.bytes().map(Tile::from_ascii).collect())
                .collect(),
        )
    }

    fn start(&self) -> Position {
        for (i, row) in self.0.iter().enumerate() {
            for (j, tile) in row.iter().enumerate() {
                if let Tile::Start = tile {
                    return Position(i, j);
                }
            }
        }
        panic!("expected start tile)")
    }

    fn at(&self, pos: Position) -> Option<Tile> {
        self.0.get(pos.0).and_then(|row| row.get(pos.1)).cloned()
    }

    fn exits(&self, from: Position) -> impl Iterator<Item = Position> + '_ {
        let tile = self.at(from).expect("valid from position");
        [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .filter(move |&dir| tile.is_open_to(dir))
        .flat_map(move |dir| {
            from.go(dir).filter(|&pos| {
                self.at(pos)
                    .is_some_and(|tile| tile.is_open_to(dir.reverse()))
            })
        })
    }
}

pub fn solve(text: &str) -> usize {
    let grid = Grid::from_str(text);
    let start = grid.start();

    let mut old = start;
    let mut new = grid
        .exits(start)
        .next()
        .expect("somewhere reachable from the start position");
    let mut len = 1; // We already took one step, from old to new.
    while new != start {
        let pos = grid
            .exits(new)
            .find(|&pos| pos != old)
            .expect("somewhere reachable from each reachable position");

        (old, new) = (new, pos);
        len += 1;
    }
    (len + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_connects() {
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

    #[test]
    fn sample() {
        for (text, want) in include_str!("samples.txt").split("\n\n").zip([4, 4, 8, 8]) {
            assert_eq!(solve(text), want);
        }
    }
}
