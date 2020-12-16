#![allow(unused_imports)]
use advent2020::error::{NoSolution, ParseError};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::str::FromStr;

fn parse_range(s: &str) -> Result<RangeInclusive<u32>, ParseError> {
    let parts: Vec<_> = s.splitn(2, '-').collect();
    if parts.len() != 2 {
        Err(ParseError::new(format!("bad range '{}'", s)))
    } else {
        Ok(parts[0].parse()?..=parts[1].parse()?)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Rule {
    field: String,
    ranges: (RangeInclusive<u32>, RangeInclusive<u32>),
}

impl Rule {
    fn is_valid(&self, value: u32) -> bool {
        self.ranges.0.contains(&value) || self.ranges.1.contains(&value)
    }
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sep = ": ";
        let pos = s.find(": ").ok_or_else(|| {
            let what = format!(r#"bad rule; expected separator: "{}""#, s);
            ParseError::new(what)
        })?;
        let (field, tail) = s.split_at(pos + sep.len());
        let parts = tail.split(' ').collect::<Vec<_>>();
        if let [first, "or", second] = parts.as_slice() {
            Ok(Rule {
                field: field[0..(field.len() - sep.len())].to_owned(),
                ranges: (parse_range(first)?, parse_range(second)?),
            })
        } else {
            Err(ParseError::new(format!("bad rule: '{}'", s)))
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
    ticket: Ticket,       // your ticket
    tickets: Vec<Ticket>, // nearby tickets
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
    let mut lines = lines.skip(1); // "your ticket:"
    let ticket = if let Some(line) = lines.next() {
        line?.parse()?
    } else {
        let what = "expected ticket, got EOF".to_owned();
        return Err(Box::new(ParseError::new(what)));
    };
    let mut tickets = Vec::new();
    for line in lines.skip(2) {
        tickets.push(line?.parse()?);
    }
    Ok(Document {
        rules,
        ticket,
        tickets,
    })
}

fn solve_part1(doc: &Document) -> u32 {
    doc.tickets
        .iter()
        .flat_map(|ticket| ticket.values.iter())
        .filter(|&value| !doc.rules.iter().any(|rule| rule.is_valid(*value)))
        .sum()
}

fn collect_valid_tickets(doc: &Document) -> impl Iterator<Item = &Ticket> {
    doc.tickets.iter().filter(move |ticket| {
        !ticket
            .values
            .iter()
            .any(|value| doc.rules.iter().all(|rule| !rule.is_valid(*value)))
    })
}

/// Maps columns (by index) to sets of rules that reject any values in them.
fn exclude_rules_by_column(doc: &Document) -> Vec<HashSet<&Rule>> {
    let column_count = doc.ticket.values.len();
    let mut excluded_rules = vec![HashSet::new(); column_count];
    for (column, &value) in
        collect_valid_tickets(doc).flat_map(|ticket| ticket.values.iter().enumerate())
    {
        let rules = doc.rules.iter().filter(|rule| !rule.is_valid(value));
        excluded_rules[column].extend(rules);
    }
    excluded_rules
}

fn solve_part2(doc: &Document) -> Result<u32, NoSolution> {
    let excluded_rules = exclude_rules_by_column(doc);
    for (column, rules) in excluded_rules.iter().enumerate() {
        println!("column {:2} excludes {:2} rules", column, rules.len());
    }
    Ok(0) // TODO
}

fn main() {
    let input_path = "tests/day16/input";
    match load_document(input_path) {
        Ok(doc) => {
            println!("{}", solve_part1(&doc));
            match solve_part2(&doc) {
                Ok(answer) => println!("{}", answer),
                Err(err) => {
                    eprintln!("error: {}: part 2: {}", input_path, err);
                    std::process::exit(2);
                }
            }
        }
        Err(err) => {
            eprintln!("error: {}: can't load document: {}", input_path, err);
            std::process::exit(3);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample1() {
        match load_document("tests/day16/sample1") {
            Ok(doc) => assert_eq!(71, solve_part1(&doc)),
            Err(err) => panic!("{}", err),
        }
    }

    #[test]
    fn part2_sample2() {
        match load_document("tests/day16/sample2") {
            Ok(doc) => assert_eq!(1, solve_part2(&doc).unwrap()),
            Err(err) => panic!("{}", err),
        }
    }
}
