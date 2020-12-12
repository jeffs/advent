use advent2020::day12::{Instruction, Ship, Waypoint};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn solve_part1<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    let mut ship = Ship::new();
    for line in BufReader::new(File::open(input)?).lines() {
        match Instruction::parse(line?)? {
            Instruction::North { distance } => ship = ship.north(distance),
            Instruction::South { distance } => ship = ship.south(distance),
            Instruction::East { distance } => ship = ship.east(distance),
            Instruction::West { distance } => ship = ship.west(distance),
            Instruction::Left { degrees } => ship = ship.left(degrees),
            Instruction::Right { degrees } => ship = ship.right(degrees),
            Instruction::Forward { distance } => ship = ship.forward(distance),
        }
    }
    Ok(ship.distance())
}

#[allow(dead_code, unused_mut, unused_variables)]
fn solve_part2<P: AsRef<Path>>(input: P) -> Result<usize, Box<dyn Error>> {
    let mut ship = Ship::new();
    let mut way = Waypoint::new();
    let mut i = 0;
    println!("{}. ship: {:?}\n   way:  {:?}", i, ship, way);
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        match Instruction::parse(line.clone())? {
            Instruction::North { distance } => way = way.north(distance),
            Instruction::South { distance } => way = way.south(distance),
            Instruction::East { distance } => way = way.east(distance),
            Instruction::West { distance } => way = way.west(distance),
            Instruction::Left { degrees } => way = way.left(ship.pos(), degrees),
            Instruction::Right { degrees } => way = way.right(ship.pos(), degrees),
            Instruction::Forward { distance } => ship = ship.toward(way.pos(), distance),
        }
        i += 1;
        println!("{}. {}\n   ship: {:?}\n   way:  {:?}", i, line, ship, way);
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
