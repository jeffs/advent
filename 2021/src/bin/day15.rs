use advent2021::{CardinalNeighbors as Neighbors, ParseError, Point};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

const ISATTY: bool = false;

type Cave = Vec<Vec<u8>>;

trait At {
    fn at(&self, p: Point) -> usize;
}

trait Set {
    fn set(&mut self, p: Point, x: usize);
}

impl At for &[Vec<u8>] {
    fn at(&self, p: Point) -> usize {
        self[p.0][p.1] as usize
    }
}

impl At for Vec<Vec<usize>> {
    fn at(&self, p: Point) -> usize {
        self[p.0][p.1]
    }
}

impl Set for Vec<Vec<usize>> {
    fn set(&mut self, p: Point, x: usize) {
        self[p.0][p.1] = x;
    }
}

fn load_cave<P>(input: P) -> Result<Cave, ParseError>
where
    P: AsRef<Path>,
{
    let mut cave = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        cave.push(line?.bytes().map(|b| b - b'0').collect());
    }
    Ok(cave)
}

fn all_points(cave: &[Vec<u8>]) -> impl Iterator<Item = Point> + 'static {
    let (m, n) = (cave.len(), cave[0].len());
    (0..m).flat_map(move |i| (0..n).map(move |j| (i, j)))
}

/// Dijkstra's algorithm, hard-coded to find a path from the top left
/// corner of a grid to the bottom right.
///
/// <https://en.wikipedia.org/wiki/Dijkstra's_algorithm>
pub fn dijkstra(cave: &[Vec<u8>]) -> usize {
    let (m, n) = (cave.len(), cave[0].len());
    let end = (m - 1, n - 1);
    let mut p = (0, 0); // current node
    let mut unvisited: HashSet<_> = all_points(cave).collect();
    let mut dist = vec![vec![usize::MAX; n]; m];
    dist.set(p, 0);
    while p != end {
        if ISATTY && unvisited.len() % 1000 == 0 {
            eprint!("\r                                    ");
            eprint!("\r{} unvisited nodes remaining", unvisited.len());
        }
        for q in Neighbors::of(cave, p).filter(|q| unvisited.contains(q)) {
            let d = dist.at(p) + cave.at(q);
            if d < dist.at(q) {
                dist.set(q, d);
            }
        }
        unvisited.remove(&p);
        p = unvisited
            .iter()
            .cloned()
            .min_by_key(|&q| dist.at(q))
            .expect("end is unreachable from start");
    }
    if ISATTY {
        eprint!("\r                                  \r");
    }
    dist.at(end)
}

pub mod part1 {
    pub use super::dijkstra as solve;

    #[cfg(test)]
    mod tests {
        use super::super::load_cave;
        use super::solve;

        #[test]
        fn test_solve_tiny() {
            [(2, 6), (3, 20)].into_iter().for_each(|(size, want)| {
                let file = format!("tests/day15/tiny{}", size);
                let cave = load_cave(file).unwrap();
                assert_eq!(want, solve(&cave));
            });
        }

        #[test]
        fn test_solve() {
            let cave = load_cave("tests/day15/sample").unwrap();
            assert_eq!(40, solve(&cave));
        }

        #[test]
        fn test_solve_sample2() {
            let cave = load_cave("tests/day15/sample2").unwrap();
            assert_eq!(315, solve(&cave));
        }
    }
}

pub mod part2 {
    use super::*;

    const GROWTH_FACTOR: usize = 5;

    fn roll(risk: u8) -> u8 {
        risk % 9 + 1
    }

    fn extend_row(row: &mut Vec<u8>) {
        let n = row.len();
        for k in 0..(GROWTH_FACTOR - 1) {
            for j in 0..n {
                row.push(roll(row[k * n + j]));
            }
        }
    }

    fn extend_right(cave: &mut Cave) {
        cave.iter_mut().for_each(extend_row);
    }

    fn extend_down(cave: &mut Cave) {
        let m = cave.len();
        for k in 0..(GROWTH_FACTOR - 1) {
            for i in 0..m {
                cave.push(cave[k * m + i].iter().cloned().map(roll).collect());
            }
        }
    }

    fn _print_cave(cave: &[Vec<u8>]) {
        for row in cave {
            for risk in row {
                print!("{}", risk);
            }
            println!();
        }
    }

    pub fn solve(cave: &mut Cave) -> usize {
        extend_right(cave);
        extend_down(cave);
        dijkstra(cave)
    }

    #[cfg(test)]
    mod tests {
        use super::super::load_cave;
        use super::solve;

        #[test]
        fn test_solve() {
            let mut cave = load_cave("tests/day15/sample").unwrap();
            assert_eq!(315, solve(&mut cave));
        }
    }
}

fn main() {
    let input = "tests/day15/input";
    let mut cave = load_cave(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&cave));
    println!("{}", part2::solve(&mut cave));
}
