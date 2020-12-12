use advent2020::day12::vector::{EAST, NORTH, SOUTH, WEST};
use advent2020::day12::Vector;
use advent2020::day12::{Instruction, Ship};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve_part1<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    let mut ship = Ship::new();
    for line in BufReader::new(File::open(input)?).lines() {
        ship = match Instruction::parse(line?)? {
            Instruction::North { distance } => ship.strafe(NORTH * distance as isize),
            Instruction::South { distance } => ship.strafe(SOUTH * distance as isize),
            Instruction::East { distance } => ship.strafe(EAST * distance as isize),
            Instruction::West { distance } => ship.strafe(WEST * distance as isize),
            Instruction::Left { degrees } => ship.turn(degrees as isize),
            Instruction::Right { degrees } => ship.turn(-(degrees as isize)),
            Instruction::Forward { distance } => ship.forward(distance),
        };
    }
    Ok(ship.distance())
}

fn solve_part2<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    let mut ship = Ship::new();
    let mut way = Vector { dx: 10, dy: 1 };
    for line in BufReader::new(File::open(input)?).lines() {
        match Instruction::parse(line?)? {
            Instruction::North { distance } => way += NORTH * distance as isize,
            Instruction::South { distance } => way += SOUTH * distance as isize,
            Instruction::East { distance } => way += EAST * distance as isize,
            Instruction::West { distance } => way += WEST * distance as isize,
            Instruction::Left { degrees } => way = way.rotate(degrees as isize),
            Instruction::Right { degrees } => way = way.rotate(-(degrees as isize)),
            Instruction::Forward { distance } => {
                ship = ship.wayward(way, distance);
            }
        }
    }
    Ok(ship.distance())
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
