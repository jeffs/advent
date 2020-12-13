use super::vector::{EAST, NORTH, SOUTH, WEST};
use super::Vector;
use super::{Instruction, Ship};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(286, solve("tests/day12/sample1").unwrap());
    }
}
