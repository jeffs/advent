use crate::step::Step;

use advent2021::ParseError;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct Puzzle {
    pub steps: Vec<Step>,
}

impl Puzzle {
    pub fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut steps = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            steps.push(line?.parse()?);
        }
        Ok(Puzzle { steps })
    }
}
