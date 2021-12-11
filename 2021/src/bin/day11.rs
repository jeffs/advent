use std::collections::HashSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day11 {
    use super::*;

    type Point = (usize, usize);

    const E: u8 = 10; // energy level at which octopuses flash

    #[derive(Clone, Eq, PartialEq)]
    pub struct Grid(Vec<Vec<u8>>);

    impl Grid {
        fn at(&self, p: Point) -> u8 {
            self.0[p.0][p.1]
        }

        fn dim(&self) -> Point {
            (self.0.len(), self.0[0].len())
        }

        fn points(&self) -> impl Iterator<Item = Point> {
            let (m, n) = self.dim();
            (0..m)
                .into_iter()
                .flat_map(move |i| (0..n).into_iter().map(move |j| (i, j)))
        }

        fn set(&mut self, p: Point, value: u8) {
            self.0[p.0][p.1] = value;
        }

        fn flashpoints(&self) -> impl Iterator<Item = Point> + '_ {
            self.points().filter(|&p| self.at(p) >= E)
        }

        fn increment_all(&mut self) {
            self.0
                .iter_mut()
                .flat_map(|row| row.iter_mut())
                .for_each(|cell| *cell += 1);
        }

        fn increment_neighborhood(&mut self, p: Point) {
            let (m, n) = self.dim();
            let (ia, iz) = (p.0.max(1) - 1, (p.0 + 2).min(m));
            let (ja, jz) = (p.1.max(1) - 1, (p.1 + 2).min(n));
            self.0[ia..iz]
                .iter_mut()
                .flat_map(|row| row[ja..jz].iter_mut())
                .for_each(|cell| *cell += 1);
        }

        pub fn from_file<P>(input: P) -> Result<Self, Box<dyn Error>>
        where
            P: AsRef<Path> + Clone,
        {
            let mut rows = Vec::new();
            for line in BufReader::new(File::open(&input)?).lines() {
                rows.push(line?.bytes().map(|b| b - b'0').collect());
            }
            Ok(Grid(rows))
        }

        /// Advances the grid, and returns the number of new flashes.
        fn step(&mut self) -> usize {
            self.increment_all();
            let mut seen: HashSet<Point> = HashSet::new();
            let mut news: Vec<_> = self.flashpoints().collect();
            while !news.is_empty() {
                seen.extend(news.iter().cloned());
                news.iter().for_each(|&p| self.increment_neighborhood(p));
                news.clear();
                news.extend(self.flashpoints().filter(|p| !seen.contains(p)));
            }
            let mut count = 0;
            for p in seen {
                count += 1;
                self.set(p, 0);
            }
            count
        }
    }

    impl Display for Grid {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
            for row in &self.0 {
                for cell in row {
                    write!(f, "{:X}", cell)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    pub mod part1 {
        use super::*;

        pub fn solve(mut grid: Grid) -> usize {
            let mut count = 0;
            for _ in 0..100 {
                count += grid.step();
            }
            count
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                let grid = Grid::from_file("tests/day11/sample10x10").unwrap();
                assert_eq!(1656, solve(grid));
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_grid_from_file() {
            let want = vec![
                b"11111".map(|b| b - b'0').to_vec(),
                b"19991".map(|b| b - b'0').to_vec(),
                b"19191".map(|b| b - b'0').to_vec(),
                b"19991".map(|b| b - b'0').to_vec(),
                b"11111".map(|b| b - b'0').to_vec(),
            ];
            let grid = Grid::from_file("tests/day11/sample3x3").unwrap();
            assert_eq!(want, grid.0);
        }

        #[test]
        fn test_grid_step3x3() {
            let mut grid = Grid::from_file("tests/day11/sample3x3").unwrap();
            for (rows, flash_count) in [
                ([b"34543", b"40004", b"50005", b"40004", b"34543"], 9),
                ([b"45654", b"51115", b"61116", b"51115", b"45654"], 0),
            ] {
                let rows: Vec<Vec<u8>> = rows
                    .iter()
                    .map(|row| row.iter().map(|b| b - b'0').collect())
                    .collect();
                assert_eq!(flash_count, grid.step());
                assert_eq!(rows, grid.0);
            }
        }

        #[test]
        fn test_grid_step10x10() {
            let mut grid = Grid::from_file("tests/day11/sample10x10").unwrap();
            assert_eq!(0, grid.step());
            assert_eq!(35, grid.step());
        }
    }
}

fn main() {
    let input = "tests/day11/input";
    let grid = day11::Grid::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day11::part1::solve(grid));
}
