use crate::error::ParseError;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

type RuleMap = HashMap<usize, Pattern>;

#[derive(Debug)]
enum Atom {
    RuleId(usize),
    Literal(String),
}

impl Atom {
    /// Returns the number of leading bytes in the specified line corresponding
    /// to each possible match by this atom, or an empty set if there is no way
    /// for this atom to match a prefix of the line.
    fn count_bytes(&self, line: &str, rules: &RuleMap) -> HashSet<usize> {
        match self {
            Atom::RuleId(id) => {
                let pattern = rules.get(id).expect(&format!("can't find rule id {}", id));
                pattern.count_bytes(line, rules)
            }
            Atom::Literal(prefix) if line.starts_with(prefix) => {
                [prefix.len()].iter().cloned().collect()
            }
            _ => HashSet::new(),
        }
    }
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

/// Returns the number of leading bytes in the specified line corresponding to
/// each possible match by the specified sequence of atoms, or an empty set if
/// there is no way for this sequence to match a prefix of the line.
fn count_bytes(atoms: &[Atom], line: &str, rules: &RuleMap) -> HashSet<usize> {
    match atoms.len() {
        0 => HashSet::new(),
        1 => atoms[0].count_bytes(line, rules),
        _ => {
            let mut counts = HashSet::new();
            for head_count in atoms[0].count_bytes(line, rules) {
                for tail_count in count_bytes(&atoms[1..], &line[head_count..], rules) {
                    counts.insert(head_count + tail_count);
                }
            }
            counts
        }
    }
}

#[derive(Debug)]
struct Branch(Vec<Atom>);

impl Branch {
    /// Returns the number of leading bytes in the specified line corresponding
    /// to each possible match by this branch, or an empty set if there is no
    /// way for this branch to match a prefix of the line.
    fn count_bytes(&self, line: &str, rules: &RuleMap) -> HashSet<usize> {
        count_bytes(&self.0, line, rules)
    }
}

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

#[derive(Debug)]
struct Pattern(Vec<Branch>); // Branches are alternative sequences of atoms.

impl Pattern {
    fn count_bytes(&self, line: &str, rules: &RuleMap) -> HashSet<usize> {
        let mut counts = HashSet::new();
        for branch in self.0.iter() {
            counts.extend(branch.count_bytes(line, &rules));
        }
        counts
    }

    fn matches(&self, line: &str, rules: &RuleMap) -> bool {
        0 != self
            .count_bytes(line, rules)
            .into_iter()
            .filter(|&count| count == line.len())
            .count()
    }
}

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
    let rules: RuleMap = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<Vec<Rule>, _>>()?
        .into_iter()
        .map(|rule| (rule.id, rule.pattern))
        .collect();
    let pattern = rules
        .get(&0)
        .ok_or_else(|| ParseError::new("can't find rule 0"))?;
    Ok(lines.filter(|line| pattern.matches(line, &rules)).count())
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
