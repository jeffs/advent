#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use advent2021::{EmptyFile, NoSolution, ParseError};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

mod day5 {
    use super::*;

    struct Point {
        x: usize,
        y: usize,
    }

    impl FromStr for Point {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some((x, y)) = s.split_once(',') {
                Ok(Point {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            } else {
                Err(ParseError::new(format!("bad point: {}", s)))
            }
        }
    }

    pub struct Segment {
        p1: Point,
        p2: Point,
    }

    impl FromStr for Segment {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some((p1, p2)) = s.split_once(" -> ") {
                Ok(Segment {
                    p1: p1.parse()?,
                    p2: p2.parse()?,
                })
            } else {
                Err(ParseError::new(format!("bad segment: {}", s)))
            }
        }
    }

    struct Floor {
        counts: Vec<u32>, // number of overlapping vents at each point
        width: usize,
    }

    pub fn load_segments<P: AsRef<Path>>(input: P) -> Result<Vec<Segment>, Box<dyn Error>> {
        let mut segments = Vec::new();
        for line in BufReader::new(File::open(&input)?).lines() {
            segments.push(line?.parse()?);
        }
        Ok(segments)
    }

    pub mod part1 {
        use super::*;

        pub fn solve(segments: &[Segment]) -> Result<u64, NoSolution> {
            Err(todo!())
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                assert_eq!(5, solve(load_segments("tests/day5/sample").unwrap()));
            }
        }
    }
}

fn main() {
    let input = "tests/day5/input";
    let segments = day5::load_segments("tests/day5/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    match day5::part1::solve(&segments) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
}
