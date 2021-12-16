#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use advent2021::{EmptyFile, NoSolution, ParseError};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

fn load_fish<P: AsRef<Path>>(input: P) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut fish = Vec::new();
    for line in BufReader::new(File::open(&input)?).lines() {
        for field in line?.split(',') {
            fish.push(field.parse()?);
        }
    }
    Ok(fish)
}

fn next(old: HashMap<u8, usize>) -> HashMap<u8, usize> {
    let mut new: HashMap<u8, usize> = HashMap::new();
    for (&timer, &count) in old.iter() {
        if timer != 0 {
            new.insert(timer - 1, count);
        }
    }
    if let Some(&noobs) = old.get(&0) {
        new.insert(8, noobs);
        new.entry(6)
            .and_modify(|count| *count += noobs)
            .or_insert(noobs);
    }
    new
}

fn solve_days(fish: &[u8], days: u32) -> usize {
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for &f in fish {
        *counts.entry(f).or_default() += 1
    }
    for _ in 0..days {
        counts = next(counts)
    }
    counts.values().sum()
}

pub mod part1 {
    use super::*;

    pub fn solve(fish: &[u8]) -> usize {
        solve_days(fish, 80)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(5934, solve(&load_fish("tests/day6/sample").unwrap()));
        }
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(fish: &[u8]) -> usize {
        solve_days(fish, 256)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(26984457539, solve(&load_fish("tests/day6/sample").unwrap()));
        }
    }
}

fn main() {
    let input = "tests/day6/input";
    let fish = load_fish("tests/day6/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&fish));
    println!("{}", part2::solve(&fish));
}
