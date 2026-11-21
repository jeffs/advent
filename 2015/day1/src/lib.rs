//! The 2015 day 1 puzzle.

pub mod elevator;
pub mod solution;
pub mod walk;

use std::fmt::Display;

use puzzle::Puzzle;
use solution::Input;

pub struct Day1;

impl Puzzle for Day1 {
    const YEAR: u16 = 2015;

    const DAY: u8 = 1;

    type Input = Input;

    fn part1(input: &Input) -> impl Display {
        solution::part1(input)
    }

    fn part2(input: &Input) -> impl Display {
        solution::part2(input)
    }
}
