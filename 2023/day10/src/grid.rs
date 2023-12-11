use crate::{direction::Direction, position::Position, tile::Tile};

#[derive(Clone)]
pub struct Grid(Vec<Vec<Tile>>);

fn expand_east(row: Vec<Tile>) -> Vec<Tile> {
    row.into_iter()
        .flat_map(|tile| [tile, tile.expand_east()])
        .collect()
}

fn expand_south(row: Vec<Tile>) -> [Vec<Tile>; 2] {
    let new_row = row.iter().cloned().map(Tile::expand_south).collect();
    [row, new_row]
}

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

    pub fn set(&mut self, pos: Position, tile: Tile) {
        self.0[pos.0][pos.1] = tile;
    }

    pub fn at(&self, pos: Position) -> Option<Tile> {
        self.0.get(pos.0).and_then(|row| row.get(pos.1)).cloned()
    }

    pub fn exits(&self, from: Position) -> impl Iterator<Item = Position> + '_ {
        let tile = self.at(from).expect("valid from position");
        Direction::iter()
            .filter(move |&dir| tile.is_open_to(dir))
            .flat_map(move |dir| {
                from.go(dir).filter(|&pos| {
                    self.at(pos)
                        .is_some_and(|tile| tile.is_open_to(dir.reverse()))
                })
            })
    }

    pub fn is_ground(&self, pos: Position) -> bool {
        self.at(pos).is_some_and(Tile::is_ground)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Position, Tile)> + '_ {
        self.0.iter().enumerate().flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, tile)| (Position(i, j), *tile))
        })
    }

    pub fn expand(self) -> Grid {
        Grid(
            self.0
                .into_iter()
                .map(expand_east)
                .flat_map(expand_south)
                .collect(),
        )
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0.get(0).map(|row| row.len()).unwrap_or_default()
    }

    pub fn to_ascii(&self) -> Vec<Vec<u8>> {
        self.0
            .iter()
            .map(|row| row.iter().map(|tile| tile.to_ascii()).collect())
            .collect()
    }
}
