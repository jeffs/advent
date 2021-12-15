//! Starter program for Advent of Code solutions.
//!
//! This file includes a basic module structure and file parsing.  Copy it,
//! replace the module name day_nn, and use it as a starting point for your
//! Rust solution to a particular Advent of Code problem.

use advent2021::ParseError;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day_nn {
    use super::*;

    pub struct Puzzle {
        numbers: Vec<i32>,
    }

    pub fn load_puzzle<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut numbers = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            numbers.push(line?.parse()?);
        }
        Ok(Puzzle { numbers })
    }

    pub mod part1 {
        use super::*;

        pub fn solve(puzzle: &Puzzle) -> i32 {
            puzzle.numbers.iter().sum()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_puzzle;
            use super::solve;

            #[test]
            fn test_solve() {
                let puzzle = load_puzzle("tests/day_nn/sample").unwrap();
                assert_eq!(17, solve(&puzzle));
            }
        }
    }
}

fn main() {
    let input = "tests/day_nn/input";
    let puzzle = day_nn::load_puzzle(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day_nn::part1::solve(&puzzle));
}
