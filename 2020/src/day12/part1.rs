use super::vector::{EAST, NORTH, SOUTH, WEST};
use super::{Instruction, Ship};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(25, solve("tests/day12/sample1").unwrap());
    }
}
