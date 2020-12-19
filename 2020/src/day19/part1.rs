#![allow(dead_code, unused_mut, unused_variables)]

use crate::error::ParseError;
use std::collections::HashMap;
use std::str::FromStr;

enum Atom {
    RuleId(usize),
    Literal(String),
}

impl FromStr for Atom {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 2 && s.starts_with("\"") && s.ends_with("\"") {
            Ok(Atom::Literal(s[1..(s.len() - 1)].to_owned()))
        } else {
            Ok(Atom::RuleId(s.parse()?))
        }
    }
}

struct Branch(Vec<Atom>);

impl FromStr for Branch {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let atoms = s
            .split_whitespace()
            .map(|token| token.parse())
            .collect::<Result<_, _>>()?;
        Ok(Branch(atoms))
    }
}

struct Pattern(Vec<Branch>); // branches are alternative sequences of atoms

impl FromStr for Pattern {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let branches = s
            .trim_start()
            .split(" | ")
            .map(|branch| branch.parse())
            .collect::<Result<_, _>>()?;
        Ok(Pattern(branches))
    }
}

struct Rule {
    id: usize,
    pattern: Pattern,
}

impl FromStr for Rule {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id_pattern = s.split(':').collect::<Vec<_>>();
        let (id, pattern) = match id_pattern.as_slice() {
            [id, pattern] => (id.parse()?, pattern.parse()?),
            _ => {
                let what = format!("pattern needs colon: {}", s);
                return Err(ParseError::new(what));
            }
        };
        Ok(Rule { id, pattern })
    }
}

pub fn solve(text: &str) -> Result<usize, ParseError> {
    let mut lines = text.lines();
    let rules: HashMap<usize, Pattern> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<Vec<Rule>, _>>()?
        .into_iter()
        .map(|rule| (rule.id, rule.pattern))
        .collect();
    for line in lines {
        todo!()
    }
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let text = fs::read_to_string("tests/day19/sample1").unwrap();
        assert_eq!(2, solve(&text).unwrap());
    }
}
