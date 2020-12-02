use std::collections::HashSet;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

#[derive(Debug)]
struct NoSolution;

impl Display for NoSolution {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "no solution")
    }
}

impl Error for NoSolution {}

mod part1 {
    use super::*;

    fn load_entries<P: AsRef<Path>>(input: P) -> Result<HashSet<i32>, Box<dyn Error>> {
        let mut entries = HashSet::new();
        for line in BufReader::new(File::open(input)?).lines() {
            entries.insert(line?.parse()?);
        }
        Ok(entries)
    }

    pub fn solve<P: AsRef<Path>>(input: P) -> Result<i32, Box<dyn Error>> {
        let entries = load_entries(input)?;
        for entry in &entries {
            let delta = 2020 - entry;
            if entries.contains(&delta) {
                return Ok(entry * delta);
            }
        }
        Err(Box::new(NoSolution))
    }
}

mod part2 {
    use super::*;

    pub fn solve<P: AsRef<Path>>(_input: P) -> Result<i32, Box<dyn Error>> {
        todo!()
    }
}

fn main() {
    let input = "tests/day1/input";
    let answer1 = part1::solve(input).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });
    println!("{}", answer1);
    let answer2 = part2::solve(input).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(2);
    });
    println!("{}", answer2);
}
