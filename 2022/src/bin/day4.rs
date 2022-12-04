use advent2022::{BoxedError, StaticError};
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;
use std::process::exit;

struct Assignment {
    first: u64,
    last: u64,
}

impl Assignment {
    fn contains(&self, section: u64) -> bool {
        self.first <= section && section <= self.last
    }

    fn fully_contains(&self, other: &Assignment) -> bool {
        self.first <= other.first && other.last <= self.last
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains(other.first) || self.contains(other.last) || other.fully_contains(self)
    }
}

fn parse_range(range: &str) -> Result<Assignment, BoxedError> {
    let mut split = range.split('-');
    let pair = match (split.next(), split.next(), split.next()) {
        (Some(first), Some(last), None) => (first, last),
        _ => return Err(StaticError::boxed("bad range")),
    };
    let first = pair.0.parse()?;
    let last = pair.1.parse()?;
    Ok(Assignment { first, last })
}

fn parse_range_pair(line: &str) -> Result<(Assignment, Assignment), BoxedError> {
    let mut split = line.split(',');
    let ranges = match (split.next(), split.next(), split.next()) {
        (Some(first), Some(second), None) => (first, second),
        _ => return Err(StaticError::boxed("bad range pair")),
    };
    let first = parse_range(ranges.0)?;
    let second = parse_range(ranges.1)?;
    Ok((first, second))
}

pub mod part1 {
    use super::*;

    pub fn solve(path: impl AsRef<Path>) -> Result<u64, BoxedError> {
        let mut sum = 0;
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let (first, second) = parse_range_pair(&line?)?;
            if first.fully_contains(&second) || second.fully_contains(&first) {
                sum += 1;
            }
        }
        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let answer = solve("tests/day4/sample").expect("sample should have an answer");
            assert_eq!(2, answer);
        }

        #[test]
        fn test_solve_input() {
            let answer = solve("tests/day4/input").expect("input should have an answer");
            assert_eq!(466, answer);
        }
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(path: impl AsRef<Path>) -> Result<u64, BoxedError> {
        let mut sum = 0;
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let (first, second) = parse_range_pair(&line?)?;
            if first.overlaps(&second) {
                sum += 1;
            }
        }
        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let answer = solve("tests/day4/sample").expect("sample should have an answer");
            assert_eq!(4, answer);
        }

        #[test]
        fn test_solve_input() {
            let answer = solve("tests/day4/input").expect("input should have an answer");
            assert_eq!(865, answer);
        }
    }
}

fn main() {
    for solve in [part1::solve, part2::solve] {
        let answer = solve("tests/day4/input").unwrap_or_else(|err| {
            eprintln!("error: {err}");
            exit(1);
        });
        println!("{answer}");
    }
}
