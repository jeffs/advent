use crate::error::{EmptyFile, ParseError};
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Input {
    timestamp: usize,
    buses: Vec<usize>,
}

fn load_input(input_path: &str) -> Result<Input, Box<dyn Error>> {
    let text = fs::read_to_string(input_path)?;
    let mut lines = text.split_terminator('\n');
    let timestamp: usize = lines
        .next()
        .ok_or_else(|| EmptyFile::new(input_path))?
        .parse()?;
    let buses: Vec<usize> = lines
        .next()
        .ok_or_else(|| ParseError::in_file(input_path, "expected two lines, got only one"))?
        .split_terminator(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    if lines.next().is_some() {
        Err(Box::new(ParseError::in_file(
            input_path,
            "expected only two lines",
        )))
    } else if buses.is_empty() {
        Err(Box::new(ParseError::in_file(
            input_path,
            "can't find any buses",
        )))
    } else {
        Ok(Input { timestamp, buses })
    }
}

pub fn solve(input_path: &str) -> Result<usize, Box<dyn Error>> {
    let input = load_input(input_path)?;
    if input.buses.iter().any(|&bus| input.timestamp % bus == 0) {
        Ok(0)
    } else {
        let to_etd = |bus| bus * (input.timestamp / bus + 1);
        let etd = input.buses.iter().map(to_etd).min().unwrap();
        let index = input
            .buses
            .iter()
            .map(to_etd)
            .position(|t| t == etd)
            .unwrap();
        let bus = input.buses[index];
        Ok(bus * (etd - input.timestamp))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        assert_eq!(295, solve("tests/day13/sample1").unwrap());
    }
}
