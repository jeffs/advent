#![allow(dead_code, unused_mut, unused_variables)]

mod beacon;
mod cube;

use beacon::Beacon;
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

    fn progress(&mut self, u: &[&Cube], t: &[Cube], b: &HashSet<Beacon>) {
        if self.is_enabled {
            let [u, t, b] = [u.len(), t.len(), b.len()];
            eprintln!("{} unconnected, {} transformed, {} beacons", u, t, b);
        }
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(cubes: &[Cube]) -> usize {
        let mut log = Log::new();
        let mut unconnected = Vec::from_iter(cubes.iter().skip(1));
        let mut transformed = vec![cubes[0].clone()]; // in scanner 0's frame of reference
        let mut all_beacons = HashSet::new();
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
            all_beacons.extend(old.beacons.into_iter());
            log.progress(&unconnected, &transformed, &all_beacons);
        }
        if !unconnected.is_empty() {
            panic!("couldn't connect all cubes");
        }
        all_beacons.len()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            let cubes = cubes_from_file("tests/day19/sample").unwrap();
            assert_eq!(79, solve(&cubes));
        }
    }
}

fn main() {
    let input = "tests/day19/input";
    let cubes = cubes_from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&cubes));
}
