use advent2020::error::{NoSolution, ParseError};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn parse_range(s: &str) -> Result<RangeInclusive<u64>, ParseError> {
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
    ranges: (RangeInclusive<u64>, RangeInclusive<u64>),
}

impl Rule {
    fn is_valid(&self, value: u64) -> bool {
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
    values: Vec<u64>,
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

fn solve_part1(doc: &Document) -> u64 {
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

fn enumerate_values(ticket: &Ticket) -> impl Iterator<Item = (usize, &u64)> {
    ticket.values.iter().enumerate()
}

/// Maps columns (by index) to sets of rules that rejected any values in them.
fn exclude_rules_by_column(doc: &Document) -> Vec<HashSet<&Rule>> {
    let mut excluded_rules = vec![HashSet::new(); doc.ticket.values.len()];
    let tickets = collect_valid_tickets(doc);
    for (column, &value) in tickets.flat_map(enumerate_values) {
        let rules = doc.rules.iter().filter(|rule| !rule.is_valid(value));
        excluded_rules[column].extend(rules);
    }
    excluded_rules
}

fn complement<'doc>(
    sets: &[HashSet<&'doc Rule>],
    universe: &'doc [Rule],
) -> HashMap<usize, HashSet<&'doc Rule>> {
    let universe = HashSet::from_iter(universe.iter());
    sets.iter().map(|set| &universe - set).enumerate().collect()
}

/// Maps each rule to its column index.
fn map_columns(doc: &Document) -> Result<HashMap<&Rule, usize>, NoSolution> {
    // Map columns to sets of rules that cannot apply to them ("exclusions").
    // Complement the sets to find applicable rules by column ("candidates").
    // While candidates remain (i.e., any column is mapped to multiple rules):
    //      Find the first column having exactly one candidate.
    //          If none, return NoSolution.
    //      Graduate that entry to a match:
    //          Map the rule to the column ("columns").
    //          Remove the entry from the candidates map.
    //      Remove the matched rule from all remaining candidate sets.
    // Return the final mapping from rules to columns.
    let exclusions = exclude_rules_by_column(doc);
    let mut candidates = complement(&exclusions, &doc.rules);
    let mut columns = HashMap::new();
    while !candidates.is_empty() {
        let (&column, rule) = candidates
            .iter_mut()
            .find_map(|(column, rules)| {
                if rules.len() == 1 {
                    rules.drain().next().map(|rule| (column, rule))
                } else {
                    None
                }
            })
            .ok_or(NoSolution)?;
        candidates.remove(&column);
        columns.insert(rule, column);
        for rules in candidates.values_mut() {
            rules.remove(&rule);
        }
    }
    Ok(columns)
}

fn solve_part2(doc: &Document) -> Result<u64, NoSolution> {
    Ok(map_columns(doc)?
        .iter()
        .filter_map(|(rule, &column)| {
            if rule.field.starts_with("departure") {
                doc.ticket.values.get(column)
            } else {
                None
            }
        })
        .product())
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
