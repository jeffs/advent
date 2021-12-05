#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use advent2021::{EmptyFile, NoSolution, ParseError};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day5 {
    use super::*;

    type Point = (usize, usize);

    fn parse_point(s: &str) -> Result<Point, ParseError> {
        if let Some((x, y)) = s.split_once(',') {
            Ok((x.parse()?, y.parse()?))
        } else {
            Err(ParseError::new(format!("bad point: {}", s)))
        }
    }

    type Segment = (Point, Point);

    fn parse_segment(s: &str) -> Result<Segment, ParseError> {
        if let Some((p1, p2)) = s.split_once(" -> ") {
            Ok((parse_point(&p1)?, parse_point(&p2)?))
        } else {
            Err(ParseError::new(format!("bad segment: {}", s)))
        }
    }

    pub fn load_segments<P: AsRef<Path>>(input: P) -> Result<Vec<Segment>, Box<dyn Error>> {
        let mut segments = Vec::new();
        for line in BufReader::new(File::open(&input)?).lines() {
            segments.push(parse_segment(&line?)?);
        }
        Ok(segments)
    }

    pub mod part1 {
        use super::*;

        fn normalize(segment: &Segment) -> Segment {
            let ((x1, y1), (x2, y2)) = *segment;
            ((x1.min(x2), y1.min(y2)), (x1.max(x2), y1.max(y2)))
        }

        pub fn solve(segments: &[Segment]) -> usize {
            let mut floor: HashMap<(usize, usize), usize> = HashMap::new();
            for ((x1, y1), (x2, y2)) in segments.iter().map(normalize) {
                if x1 == x2 || y1 == y2 {
                    for y in y1..=y2 {
                        for x in x1..=x2 {
                            *floor.entry((x, y)).or_default() += 1;
                        }
                    }
                }
            }
            floor.into_values().filter(|&n| n > 1).count()
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                let segments = load_segments("tests/day5/sample").unwrap();
                assert_eq!(5, solve(&segments));
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
    println!("{}", day5::part1::solve(&segments));
}
