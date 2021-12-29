//! Starter program for Advent of Code solutions.
//!
//! This program is meant to be be copied and tweaked for each day of Advent.

use advent2021::ParseError;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

pub struct Puzzle {
    numbers: Vec<i32>,
}

impl Puzzle {
    fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut numbers = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            numbers.push(line?.parse()?);
        }
        Ok(Puzzle { numbers })
    }
}

pub mod part1 {
    use super::Puzzle;

    pub fn solve(puzzle: &Puzzle) -> i32 {
        puzzle.numbers.iter().sum() // TODO
    }

    #[cfg(test)]
    mod tests {
        // use super::{solve, Puzzle};

        // #[test]
        // fn test_solve() {
        //     let puzzle = load_puzzle("tests/dayN/sample").unwrap();
        //     assert_eq!(R, solve(&puzzle));
        // }
    }
}

fn main() {
    let input = "tests/dayN/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&puzzle));
}
