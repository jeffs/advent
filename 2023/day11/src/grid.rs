#[derive(Clone, Copy)]
struct Position(usize, usize);

#[derive(Clone, Copy)]
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
        matches!(*self, Tile::Empty)
    }

    fn is_galaxy(&self) -> bool {
        matches!(*self, Tile::Galaxy)
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

    fn galaxies(&self, expansion: usize) -> impl Iterator<Item = Position> + '_ {
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
            .filter(move |(_, t)| t.is_galaxy())
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
