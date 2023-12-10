use crate::{direction::Direction, position::Position, tile::Tile};

pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
    pub fn from_str(text: &str) -> Grid {
        Grid(
            text.lines()
                .map(|line| line.bytes().map(Tile::from_ascii).collect())
                .collect(),
        )
    }

    pub fn start(&self) -> Position {
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

    pub fn exits(&self, from: Position) -> impl Iterator<Item = Position> + '_ {
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
