//! Parses the day 2 input and computes its answers.

use std::{fmt, str::FromStr};

pub struct Input(String);

#[derive(Debug, PartialEq)]
pub enum Error {
    Empty,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Empty => f.write_str("The input is empty."),
        }
    }
}

impl FromStr for Input {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Err(Error::Empty);
        }
        Ok(Input(s.to_owned()))
    }
}

/// Placeholder: counts the input's characters.
pub fn part1(input: &Input) -> usize {
    input.0.chars().count()
}

/// Placeholder: counts the input's lines.
pub fn part2(input: &Input) -> usize {
    input.0.lines().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_input() {
        assert_eq!("\n".parse::<Input>().err(), Some(Error::Empty));
    }

    #[test]
    fn part1_counts_characters() {
        assert_eq!(part1(&"ab\ncd".parse().unwrap()), 5);
    }

    #[test]
    fn part2_counts_lines() {
        assert_eq!(part2(&"ab\ncd".parse().unwrap()), 2);
    }
}
