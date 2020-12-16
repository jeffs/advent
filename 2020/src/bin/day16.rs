use advent2020::error::ParseError;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use std::str::FromStr;

fn parse_range(s: &str) -> Result<Range<u32>, ParseError> {
    let parts: Vec<_> = s.splitn(2, '-').collect();
    if parts.len() != 2 {
        Err(ParseError::new(format!("{}: bad range", s)))
    } else {
        Ok(parts[0].parse()?..parts[1].parse()?)
    }
}

#[derive(Debug)]
struct Rule {
    ranges: (Range<u32>, Range<u32>),
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(' ').collect::<Vec<_>>()[..] {
            [_, first, "or", second] => Ok(Rule {
                ranges: (parse_range(first)?, parse_range(second)?),
            }),
            _ => Err(ParseError::new(format!("{}: bad rule", s))),
        }
    }
}

#[derive(Debug)]
struct Ticket {
    values: Vec<u32>,
}

impl FromStr for Ticket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = Vec::new();
        for part in s.split(',') {
            values.push(part.parse()?);
        }
        Ok(Ticket { values })
    }
}

#[derive(Debug)]
struct Document {
    rules: Vec<Rule>,
    tickets: Vec<Ticket>,
}

fn load_document(input_path: &str) -> Result<Document, Box<dyn Error>> {
    let mut lines = BufReader::new(File::open(input_path)?).lines();
    let mut rules = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        rules.push(line.parse()?);
    }
    let mut tickets = Vec::new();
    let lines = lines.skip(3); // “Ignore your ticket for now.”
    for line in lines.skip(1) {
        tickets.push(line?.parse()?);
    }
    Ok(Document { rules, tickets })
}

fn solve_part1(doc: &Document) -> u32 {
    println!("{:?}", doc);
    todo!()
}

fn main() {
    let doc = load_document("tests/day16/input").unwrap();
    println!("{}", solve_part1(&doc));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample1() {
        let doc = load_document("tests/day16/sample1").unwrap();
        assert_eq!(71, solve_part1(&doc));
    }
}
