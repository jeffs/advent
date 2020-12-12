#![allow(dead_code, unused_mut, unused_variables)]

use advent2020::day12::{CardinalDirection, Instruction, RelativeDirection};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve_part1<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    let mut dir: CardinalDirection = CardinalDirection::East;
    let mut x: isize = 0;
    let mut y: isize = 0;
    for line in BufReader::new(File::open(input)?).lines() {
        match Instruction::parse(line?)? {
            Instruction::North { distance } => y += distance as isize,
            Instruction::South { distance } => y -= distance as isize,
            Instruction::East { distance } => x += distance as isize,
            Instruction::West { distance } => x -= distance as isize,
            Instruction::Left { degrees } => dir = dir.turn(RelativeDirection::Left, degrees),
            Instruction::Right { degrees } => dir = dir.turn(RelativeDirection::Right, degrees),
            Instruction::Forward { distance } => match dir {
                CardinalDirection::North => y += distance as isize,
                CardinalDirection::South => y -= distance as isize,
                CardinalDirection::East => x += distance as isize,
                CardinalDirection::West => x -= distance as isize,
            },
        }
    }
    Ok(x.abs() as usize + y.abs() as usize)
}

fn solve_part2<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    todo!()
}

fn main() {
    let input = "tests/day12/input";
    println!("{}", solve_part1(input).unwrap());
    println!("{}", solve_part2(input).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(25, solve_part1("tests/day12/sample1").unwrap());
    }

    #[test]
    fn part2_sample1() {
        assert_eq!(286, solve_part2("tests/day12/sample1").unwrap());
    }
}
