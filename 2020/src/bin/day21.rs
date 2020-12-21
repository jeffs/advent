use advent2020::error::ParseError;
use std::collections::{HashMap, HashSet};
use std::fs;

struct Analysis<'a> {
    candidates: HashMap<&'a str, HashSet<&'a str>>,
    counts: HashMap<&'a str, usize>, // ingredient => number of appearances
    safe: HashSet<&'a str>,          // ingredients that definitely have no allergens
}

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

fn analyze(text: &str) -> Result<Analysis, ParseError> {
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
    Ok(Analysis {
        candidates,
        counts,
        safe,
    })
}

fn solve_part1(analysis: &Analysis) -> Result<usize, ParseError> {
    let Analysis { counts, safe, .. } = analysis;
    Ok(safe.iter().map(|ingredient| counts[ingredient]).sum())
}

fn solve_part2(analysis: &Analysis) -> String {
    let Analysis {
        candidates, safe, ..
    } = analysis;
    let mut candidates = candidates.clone();
    for set in candidates.values_mut() {
        *set = &*set - safe;
    }
    let mut pairs: Vec<(&str, &str)> = Vec::new(); // (allergen, ingredient)...
    while !candidates.is_empty() {
        let known: HashMap<&str, &str> = candidates // allergen => ingredient
            .iter()
            .filter(|(_, set)| set.len() == 1)
            .map(|(allergen, set)| (allergen.clone(), set.iter().next().unwrap().clone()))
            .collect();
        candidates.retain(|_, set| set.len() > 1);
        for set in candidates.values_mut() {
            for ingredient in known.values() {
                set.remove(ingredient);
            }
        }
        pairs.extend(known);
    }
    pairs.sort();
    let ingredients: Vec<_> = pairs.iter().map(|pair| pair.1).collect();
    ingredients.join(",")
}

fn main() {
    let text = fs::read_to_string("tests/day21/input").unwrap();
    let analysis = analyze(&text).unwrap();
    println!("{}", solve_part1(&analysis).unwrap());
    println!("{}", solve_part2(&analysis));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_part1_sample1() {
        let text = fs::read_to_string("tests/day21/sample1").unwrap();
        let analysis = analyze(&text).unwrap();
        assert_eq!(5, solve_part1(&analysis).unwrap());
    }

    #[test]
    fn solve_part2_sample1() {
        let text = fs::read_to_string("tests/day21/sample1").unwrap();
        let analysis = analyze(&text).unwrap();
        assert_eq!("mxmxvkd,sqjhc,fvjkl", solve_part2(&analysis));
    }
}
