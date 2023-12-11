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
}

fn offsets<'a, I, J>(tileses: I, expansion: usize) -> Vec<usize>
where
    I: Iterator<Item = J>,
    J: IntoIterator<Item = &'a Tile>,
{
    tileses
        .scan(0, |gap, tiles| {
            if tiles.into_iter().all(Tile::is_empty) {
                *gap = *gap + expansion - 1;
            }
            Some(*gap)
        })
        .collect()
}

pub struct Grid(Vec<Vec<Tile>>);

impl Grid {
    pub fn parse(s: &str) -> Grid {
        Grid(
            s.lines()
                .map(|line| line.bytes().map(Tile::from_ascii).collect())
                .collect(),
        )
    }

    fn rows(&self) -> impl Iterator<Item = &Vec<Tile>> {
        self.0.iter()
    }

    fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = &Tile>> {
        (0..self.width()).map(move |j| self.rows().map(move |row| &row[j]))
    }

    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0.first().map(|row| row.len()).unwrap_or_default()
    }

    fn enumerate(&self) -> impl Iterator<Item = (Position, Tile)> + '_ {
        let (h, w) = (self.height(), self.width());
        (0..h)
            .flat_map(move |i| (0..w).map(move |j| Position(i, j)))
            .map(|p| (p, self.0[p.0][p.1]))
    }

    fn galaxies_with_expansion(&self, expansion: usize) -> impl Iterator<Item = Position> + '_ {
        let row_offsets = offsets(self.rows(), expansion);
        let column_offsets = offsets(self.columns(), expansion);
        self.enumerate()
            .filter(move |(_, t)| !t.is_empty())
            .map(move |(Position(i, j), _)| Position(i + row_offsets[i], j + column_offsets[j]))
    }

    pub fn distance_with_expansion(&self, expansion: usize) -> usize {
        let galaxies: Vec<Position> = self.galaxies_with_expansion(expansion).collect();
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
        let grid = Grid::parse(include_str!("sample.txt"));
        assert_eq!(grid.distance_with_expansion(2), 374);
        assert_eq!(grid.distance_with_expansion(10), 1030);
        assert_eq!(grid.distance_with_expansion(100), 8410);
    }
}
