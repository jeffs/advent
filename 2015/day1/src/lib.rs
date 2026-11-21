pub mod solution;

use std::fmt::Display;

use puzzle::Puzzle;
use solution::Input;

pub struct Day1;

impl Puzzle for Day1 {
    const DAY: u8 = 1;

    type Input = Input;

    fn part1(input: &Input) -> impl Display {
        solution::part1(input)
    }

    fn part2(input: &Input) -> impl Display {
        solution::part2(input)
    }
}
