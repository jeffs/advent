use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

fn parse_char(c: char) -> Result<usize, String> {
    match c {
        'F' | 'L' => Ok(0),
        'B' | 'R' => Ok(1),
        _ => Err(format!("{}: expected any of FBLR", c)),
    }
}

fn parse(seat: &str) -> Result<usize, String> {
    let mut value = 0;
    for c in seat.chars() {
        value = (value << 1) | parse_char(c)?;
    }
    Ok(value)
}

fn load_seats<P>(input: P) -> Result<Vec<usize>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut seats = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        seats.push(parse(&line?)?);
    }
    if seats.is_empty() {
        Err("empty input".into())
    } else {
        Ok(seats)
    }
}

fn solve_part1(seats: &[usize]) -> Option<usize> {
    seats.iter().cloned().max()
}

fn solve_part2(seats: &[usize]) -> Option<usize> {
    const MAX_ID: usize = (1 << 10) - 1; // seats have ten-bit IDs
    let taken: HashSet<usize> = seats.iter().cloned().collect();
    (1..MAX_ID)
        .find(|id| !taken.contains(id) && taken.contains(&(id - 1)) && taken.contains(&(id + 1)))
}

fn main() {
    let input = "tests/day5/input";
    let seats = load_seats(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", solve_part1(&seats).unwrap());
    println!("{}", solve_part2(&seats).unwrap());
}
