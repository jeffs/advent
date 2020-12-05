use std::cmp;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

fn parse_char(c: char) -> Result<u16, String> {
    match c {
        'F' | 'L' => Ok(0),
        'B' | 'R' => Ok(1),
        _ => Err(format!("{}: expected any of FBLR", c)),
    }
}

fn parse(seat: &str) -> Result<u16, String> {
    let mut value = 0;
    for c in seat.chars() {
        value = (value << 1) | parse_char(c)?;
    }
    Ok(value)
}

fn solve_part1<P>(input: P) -> Result<u16, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut lines = BufReader::new(File::open(input)?).lines();
    let mut max = parse(&lines.next().ok_or("empty input")??)?;
    for line in lines {
        max = cmp::max(max, parse(&line?)?);
    }
    Ok(max)
}

fn main() {
    let input = "tests/day5/input";
    let answer1 = solve_part1(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(1);
    });
    println!("{}", answer1);
}
