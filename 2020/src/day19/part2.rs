use super::rule::{Rule, RuleMap};
use crate::error::ParseError;

pub fn solve(text: &str) -> Result<usize, ParseError> {
    let mut lines = text.lines();
    let mut rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse())
        .collect::<Result<Vec<Rule>, _>>()?;
    rules.push("8: 42 | 42 8".parse()?);
    rules.push("11: 42 31 | 42 11 31".parse()?);
    let rules: RuleMap = rules
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
    fn solve_sample2() {
        let text = fs::read_to_string("tests/day19/sample2").unwrap();
        assert_eq!(12, solve(&text).unwrap());
    }
}
