use advent2020::error::ParseError;
use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_line(line: &str) -> Result<(HashSet<&str>, HashSet<&str>), ParseError> {
    let parts: Vec<_> = line.trim_end_matches(')').split(" (contains ").collect();
    match parts.as_slice() {
        [ingredients, allergens] => Ok((
            ingredients.split_whitespace().collect(),
            allergens
                .split_whitespace()
                .map(|a| a.trim_end_matches(','))
                .collect(),
        )),
        _ => Err(ParseError::new(
            "expected: INGREDIENTS (contains ALLERGENS)",
        )),
    }
}

fn extend_set<'a>(mut target: HashSet<&'a str>, source: &HashSet<&'a str>) -> HashSet<&'a str> {
    target.extend(source);
    target
}

fn solve_part1(text: &str) -> Result<usize, ParseError> {
    let mut candidates: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut counts = HashMap::new();
    for line in text.lines() {
        let (ingredients, allergens) = parse_line(line)?;
        for allergen in allergens.iter() {
            if let Some(set) = candidates.get_mut(allergen) {
                *set = &*set & &ingredients;
            } else {
                candidates.insert(allergen, ingredients.clone());
            }
        }
        for ingredient in ingredients {
            *counts.entry(ingredient).or_insert(0) += 1;
        }
    }
    let all_ingredients: HashSet<_> = counts.keys().cloned().collect();
    let dangerous = candidates.values().fold(HashSet::new(), extend_set);
    let safe = &all_ingredients - &dangerous;
    Ok(safe.iter().map(|ingredient| counts[ingredient]).sum())
}

fn main() {
    let input_path = "tests/day21/input";
    let text = fs::read_to_string(input_path).unwrap();
    println!("{}", solve_part1(&text).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample1() {
        let input_path = "tests/day21/sample1";
        let text = fs::read_to_string(input_path).unwrap();
        assert_eq!(5, solve_part1(&text).unwrap());
    }
}
