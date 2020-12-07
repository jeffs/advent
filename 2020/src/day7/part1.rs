use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

type Color = (String, String);
type RuleMap = HashMap<Color, HashSet<Color>>;

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
        let mut contents = HashSet::new();
        while let (Some(_count), Some(shade), Some(color), Some(_bags)) =
            (tokens.next(), tokens.next(), tokens.next(), tokens.next())
        {
            // The loop test fails if the bag contains "no" "other" "bags."
            contents.insert((shade.to_owned(), color.to_owned()));
        }
        rules.insert(key, contents);
    }
    Ok(rules)
}

mod extrapolate {
    use super::*;

    fn imp(key: &Color, directs: &RuleMap, transients: &mut RuleMap) {
        if transients.contains_key(&key) {
            return;
        }
        let children = &directs[key];
        transients.insert(key.clone(), children.clone());
        for child in children {
            imp(child, directs, transients);
            let (k, mut v) = transients.remove_entry(key).unwrap();
            v.extend(transients[child].iter().cloned());
            transients.insert(k, v);
        }
    }

    pub fn extrapolate(directs: RuleMap) -> RuleMap {
        let mut transients = RuleMap::new();
        for key in directs.keys() {
            imp(&key, &directs, &mut transients);
        }
        transients
    }
}

use extrapolate::extrapolate;

pub fn solve(input: &str, target: &Color) -> Result<usize, Box<dyn Error>> {
    Ok(extrapolate(load_rules(input)?)
        .values()
        .filter(|v| v.contains(&target))
        .count())
}
