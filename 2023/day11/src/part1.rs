#![allow(dead_code, unused_variables)]

use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
struct Position(usize, usize);

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position({}, {})", self.0, self.1)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Galaxy,
}

impl Tile {
    fn from_ascii(c: u8) -> Tile {
        match c {
            b'.' => Tile::Empty,
            b'#' => Tile::Galaxy,
            _ => panic!("{}: bad tile", c as char),
        }
    }

    fn is_empty(&self) -> bool {
        *self == Tile::Empty
    }

    fn to_ascii(self) -> u8 {
        match self {
            Tile::Empty => b'.',
            Tile::Galaxy => b'#',
        }
    }
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn from_str(s: &str) -> Grid {
        Grid(
            s.lines()
                .map(|line| line.bytes().map(Tile::from_ascii).collect())
                .collect(),
        )
    }

    fn at(&self, p: Position) -> Option<Tile> {
        self.0.get(p.0).and_then(|row| row.get(p.1)).copied()
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or_default()
    }

    fn dim(&self) -> (usize, usize) {
        (self.height(), self.width())
    }

    fn enumerate(&self) -> impl Iterator<Item = (Position, Tile)> + '_ {
        let (h, w) = self.dim();
        (0..h)
            .flat_map(move |i| (0..w).map(move |j| Position(i, j)))
            .map(|p| (p, self.0[p.0][p.1]))
    }

    fn expand(self) -> Grid {
        let mut new: Vec<Vec<Tile>> = vec![];
        for row in self.0 {
            if row.iter().all(Tile::is_empty) {
                new.push(row.clone());
            }
            new.push(row);
        }
        let w = new.first().map(|row| row.len()).unwrap_or_default();
        let empty_columns: Vec<usize> = (0..w)
            .filter(|&j| new.iter().all(|row| row[j].is_empty()))
            .collect();
        for row in &mut new {
            row.reserve(w + empty_columns.len());
            for &j in empty_columns.iter().rev() {
                assert!(row[j].is_empty());
                row.insert(j, Tile::Empty);
            }
        }
        Grid(new)
    }

    fn galaxies(&self) -> impl Iterator<Item = Position> + '_ {
        self.enumerate()
            .filter_map(|(p, t)| (t == Tile::Galaxy).then_some(p))
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.0 {
            for c in row.iter().cloned().map(Tile::to_ascii) {
                write!(f, "{}", c as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve(text: &str) -> usize {
    let grid = Grid::from_str(text);
    let grid = grid.expand();
    let galaxies: Vec<Position> = grid.galaxies().collect();
    let n = galaxies.len();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(index, p)| {
            galaxies[index + 1..]
                .iter()
                .map(|q| p.0.abs_diff(q.0) + p.1.abs_diff(q.1))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 374);
    }
}
