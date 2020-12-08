use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

type Color = (String, String);
type RuleMap = HashMap<Color, HashMap<Color, usize>>;

fn load_rules<P: AsRef<Path>>(input: P) -> Result<RuleMap, Box<dyn Error>> {
    let mut rules = RuleMap::new();
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        let mut tokens = line.split_whitespace();
        let key = (
            tokens.next().ok_or("expected shade")?.to_owned(),
            tokens.next().ok_or("expected color")?.to_owned(),
        );
        let mut tokens = tokens.skip(2); // "bags contain"
        let mut contents = HashMap::new();
        while let (Some(count), Some(shade), Some(color), Some(_bags)) =
            (tokens.next(), tokens.next(), tokens.next(), tokens.next())
        {
            // The loop test fails if the bag contains "no" "other" "bags."
            let color = (shade.to_owned(), color.to_owned());
            contents.insert(color, count.parse()?);
        }
        rules.insert(key, contents);
    }
    Ok(rules)
}

fn tally(rules: &RuleMap, color: &Color) -> usize {
    rules[color]
        .iter()
        .map(|(c, n)| n * (1 + tally(rules, c)))
        .sum()
}

pub fn solve(input: &str, color: &Color) -> Result<usize, Box<dyn Error>> {
    Ok(tally(&load_rules(input)?, color))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let rules = load_rules("tests/day7/sample1").unwrap();
        let color = ("shiny".to_owned(), "gold".to_owned());
        assert_eq!(32, tally(&rules, &color));
    }
}
