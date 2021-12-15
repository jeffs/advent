#![allow(dead_code, unused_imports, unused_variables)]

use advent2021::{CardinalNeighbors as Neighbors, ParseError, Point};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day15 {
    use super::*;

    type Cave = Vec<Vec<u8>>;

    trait At {
        fn at(&self, p: Point) -> usize;
    }

    trait Set {
        fn set(&mut self, p: Point, x: usize);
    }

    impl At for Cave {
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

    pub fn load_cave<P>(input: P) -> Result<Cave, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut cave = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            cave.push(line?.bytes().map(|b| b - b'0').collect());
        }
        Ok(cave)
    }

    pub mod part1 {
        use super::*;

        type CavePath = Vec<Point>;

        fn all_points(cave: &Cave) -> impl Iterator<Item = Point> + 'static {
            let (m, n) = (cave.len(), cave[0].len());
            (0..m).flat_map(move |i| (0..n).map(move |j| (i, j)))
        }

        fn dijkstra(cave: &Cave, start: Point, end: Point) -> usize {
            let (m, n) = (cave.len(), cave[0].len());
            let mut unvisited: HashSet<_> = all_points(cave).collect();
            let mut dist = vec![vec![usize::MAX; n]; m];
            dist.set(start, 0);
            let mut p = start; // current node
            while p != end {
                for q in Neighbors::of(cave, p).filter(|q| unvisited.contains(q)) {
                    let d = dist.at(p) + cave.at(q);
                    if d < dist.at(q) {
                        dist.set(q, d);
                    }
                }
                unvisited.remove(&p);
                p = all_points(cave)
                    .filter(|q| unvisited.contains(q))
                    .min_by_key(|&q| dist.at(q))
                    .expect("end is unreachable from start");
            }

            dist.at(end)
        }

        pub fn solve(cave: &Cave) -> usize {
            let (m, n) = (cave.len(), cave[0].len());
            let start = (0, 0);
            dijkstra(cave, start, (m - 1, n - 1))
        }

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
        }
    }
}

fn main() {
    let input = "tests/day15/input";
    let cave = day15::load_cave(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day15::part1::solve(&cave));
}
