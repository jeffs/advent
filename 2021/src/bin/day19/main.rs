#![allow(dead_code, unused_mut, unused_variables)]

mod beacon;
mod cube;

use cube::{cubes_from_file, Cube};
use std::collections::HashSet;

struct Log {
    is_enabled: bool,
}

impl Log {
    fn new() -> Log {
        Log {
            is_enabled: std::env::var("AOC_ENABLE_LOGGING")
                .ok()
                .filter(|s| !s.is_empty())
                .is_some(),
        }
    }

    fn progress(&mut self, u: &[&Cube], t: &[Cube]) {
        if self.is_enabled {
            eprintln!("{} unconnected, {} transformed", u.len(), t.len());
        }
    }
}

fn count_beacons(cubes: &[Cube]) -> usize {
    let all_beacons: HashSet<_> = cubes.iter().flat_map(|c| c.beacons()).collect();
    all_beacons.len()
}

fn max_scanner_distance(cubes: &[Cube]) -> usize {
    cubes
        .iter()
        .flat_map(|a| cubes.iter().map(|b| a.distance(b)))
        .max()
        .expect("no scanners")
}

pub fn solve(cubes: &[Cube]) -> (usize, usize) {
    let mut log = Log::new();
    let mut unconnected = Vec::from_iter(cubes.iter().skip(1));
    let mut transformed = vec![cubes[0].clone()]; // in scanner 0's frame of reference
    let mut retired = Vec::new();
    while let Some(old) = transformed.pop() {
        let mut still_unconnected = Vec::new();
        for new in unconnected {
            if let Some(cube) = old.transform(new) {
                transformed.push(cube);
            } else {
                still_unconnected.push(new);
            }
        }
        unconnected = still_unconnected;
        retired.push(old);
        log.progress(&unconnected, &transformed);
    }
    if !unconnected.is_empty() {
        panic!("couldn't connect all cubes");
    }
    (count_beacons(&retired), max_scanner_distance(&retired))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let cubes = cubes_from_file("tests/day19/sample").unwrap();
        let (answer1, answer2) = solve(&cubes);
        assert_eq!(79, answer1);
        assert_eq!(3621, answer2);
    }
}

fn main() {
    let input = "tests/day19/input";
    let cubes = cubes_from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    let (answer1, answer2) = solve(&cubes);
    println!("{}", answer1);
    println!("{}", answer2);
}
