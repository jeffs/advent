#![allow(dead_code, unused_mut, unused_variables)]

use advent2020::day12::{CardinalDirection, Instruction, Point, RelativeDirection};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve_part1<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    let mut dir = CardinalDirection::East;
    let mut pos = Point { x: 0, y: 0 };
    for line in BufReader::new(File::open(input)?).lines() {
        match Instruction::parse(line?)? {
            Instruction::North { distance } => pos = pos.north(distance),
            Instruction::South { distance } => pos = pos.south(distance),
            Instruction::East { distance } => pos = pos.east(distance),
            Instruction::West { distance } => pos = pos.west(distance),
            Instruction::Left { degrees } => dir = dir.turn(RelativeDirection::Left, degrees),
            Instruction::Right { degrees } => dir = dir.turn(RelativeDirection::Right, degrees),
            Instruction::Forward { distance } => match dir {
                CardinalDirection::North => pos = pos.north(distance),
                CardinalDirection::South => pos = pos.south(distance),
                CardinalDirection::East => pos = pos.east(distance),
                CardinalDirection::West => pos = pos.west(distance),
            },
        }
    }
    Ok(pos.x.abs() as usize + pos.y.abs() as usize)
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
