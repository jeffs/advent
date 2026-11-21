//! The 2015 day 2 puzzle.

pub mod solution;

use std::fmt::Display;

use puzzle::Puzzle;
use solution::Input;

pub struct Day2;

impl Puzzle for Day2 {
    const YEAR: u16 = 2015;

    const DAY: u8 = 2;

    type Input = Input;

    fn part1(input: &Input) -> impl Display {
        solution::part1(input)
    }

    fn part2(input: &Input) -> impl Display {
        solution::part2(input)
    }
}
