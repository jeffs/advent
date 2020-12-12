#![allow(dead_code, unused_mut, unused_variables)]

use advent2020::error::ParseError;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

enum Instruction {
    North { distance: usize },
    South { distance: usize },
    East { distance: usize },
    West { distance: usize },
    Left { degrees: usize },
    Right { degrees: usize },
    Forward { distance: usize },
}

#[derive(Clone, Copy)]
enum CardinalDirection {
    North,
    South,
    East,
    West,
}

enum RelativeDirection {
    Left,
    Right,
}

fn parse_instruction(mut line: String) -> Result<Instruction, ParseError> {
    if line.is_empty() {
        return Err(ParseError::new("empty instruction".to_owned()));
    }
    let action = line.as_bytes()[0];
    let value: usize = line.split_off(1).parse()?;
    Ok(match action {
        b'N' => Instruction::North { distance: value },
        b'E' => Instruction::East { distance: value },
        b'S' => Instruction::South { distance: value },
        b'W' => Instruction::West { distance: value },
        b'L' => Instruction::Left { degrees: value },
        b'R' => Instruction::Right { degrees: value },
        b'F' => Instruction::Forward { distance: value },
        _ => return Err(ParseError::new(format!("{}: bad action", action))),
    })
}

fn turn(dir: CardinalDirection, rel: RelativeDirection, degrees: usize) -> CardinalDirection {
    use CardinalDirection::*;
    use RelativeDirection::*;
    const CARDINALS: [CardinalDirection; 4] = [East, North, West, South];
    let old = match dir {
        North => 1,
        East => 0,
        South => 3,
        West => 2,
    };
    let rot = degrees / 90 % 4;
    match rel {
        Left => CARDINALS[(old + rot) % 4],
        Right => CARDINALS[(old + (4 - rot)) % 4],
    }
}

fn solve_part1<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    let mut dir: CardinalDirection = CardinalDirection::East;
    let mut x: isize = 0;
    let mut y: isize = 0;
    for line in BufReader::new(File::open(input)?).lines() {
        match parse_instruction(line?)? {
            Instruction::North { distance } => y += distance as isize,
            Instruction::South { distance } => y -= distance as isize,
            Instruction::East { distance } => x += distance as isize,
            Instruction::West { distance } => x -= distance as isize,
            Instruction::Left { degrees } => dir = turn(dir, RelativeDirection::Left, degrees),
            Instruction::Right { degrees } => dir = turn(dir, RelativeDirection::Right, degrees),
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
