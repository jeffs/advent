use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Position(pub usize, pub usize);

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
pub enum Tile {
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

pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
    pub fn from_str(s: &str) -> Grid {
        Grid(
            s.lines()
                .map(|line| line.bytes().map(Tile::from_ascii).collect())
                .collect(),
        )
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

    pub fn galaxies(&self, expansion: usize) -> impl Iterator<Item = Position> + '_ {
        // Cumulative expansion
        let row_gaps: Vec<usize> = self
            .0
            .iter()
            .scan(0, |gap, row| {
                if row.iter().all(Tile::is_empty) {
                    *gap = *gap + expansion - 1;
                }
                Some(*gap)
            })
            .collect();
        let column_gaps: Vec<usize> = (0..self.width())
            .scan(0, |gap, j| {
                if self.0.iter().all(|row| row[j].is_empty()) {
                    *gap = *gap + expansion - 1;
                }
                Some(*gap)
            })
            .collect();

        self.enumerate()
            .filter(move |(_, t)| (*t == Tile::Galaxy))
            .map(move |(Position(i, j), _)| Position(i + row_gaps[i], j + column_gaps[j]))
    }

    pub fn distance(&self, expansion: usize) -> usize {
        let galaxies: Vec<Position> = self.galaxies(expansion).collect();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let grid = Grid::from_str(include_str!("sample.txt"));
        assert_eq!(grid.distance(2), 374);
        assert_eq!(grid.distance(10), 1030);
        assert_eq!(grid.distance(100), 8410);
    }
}
