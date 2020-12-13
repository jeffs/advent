use advent2020::error::{EmptyFile, ParseError};
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Input1 {
    timestamp: usize,
    buses: Vec<usize>,
}

fn load_input1(input_path: &str) -> Result<Input1, Box<dyn Error>> {
    let text = fs::read_to_string(input_path)?;
    let mut lines = text.split_terminator('\n');
    let timestamp: usize = lines
        .next()
        .ok_or_else(|| EmptyFile::new(input_path))?
        .parse()?;
    let buses: Vec<usize> = lines
        .next()
        .ok_or_else(|| {
            let what = format!("{}: expected two lines, got only one", input_path);
            ParseError::new(what)
        })?
        .split_terminator(',')
        .filter_map(|s| s.parse().ok())
        .collect();
    if buses.is_empty() {
        let what = format!("{}: can't find any buses", input_path);
        Err(Box::new(ParseError::new(what)))
    } else {
        Ok(Input1 { timestamp, buses })
    }
}

fn solve_part1(input_path: &str) -> Result<usize, Box<dyn Error>> {
    let input = load_input1(input_path)?;
    if let Some(_) = input.buses.iter().find(|&&bus| input.timestamp % bus == 0) {
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

fn main() {
    let input = "tests/day13/input";
    println!("{}", solve_part1(input).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(295, solve_part1("tests/day13/sample1").unwrap());
    }
}
