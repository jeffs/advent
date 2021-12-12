use advent2021::ParseError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day12 {
    use super::*;

    struct Cave {
        name: String,
    }

    type CaveSet = HashSet<Cave>;

    struct CaveMap {
        // starts: CaveSet, // entrances to the cave system
        // ends: CaveSet,   // exits from the cave system
        neighbors: HashMap<Cave, CaveSet>,
    }

    pub fn load_caves<P>(input: P) -> Result<CaveMap, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let mut neighbors = HashMap::new();
        for line in BufReader::new(File::open(input)?).lines() {
            let line = line?;
            let (left, right) = line.split_once('-').ok_or_else(|| {
                let what = format!("bad line: {}", line);
                ParseError::new(what)
            })?;
            neighbors
                .entry(left)
                .and_modify(|set: &mut CaveSet| set.insert(right.to_string()))
                .or_insert_with(|| [right].collect());
        }
        Ok(CaveMap { neighbors })
    }

    pub mod part1 {
        pub fn solve(_caves: &[Vec<u8>]) -> usize {
            todo!()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_caves;
            use super::solve;

            #[test]
            fn test_solve() {
                let wants = [10, 19, 226];
                for (index, want) in wants.into_iter().enumerate() {
                    let file = format!("tests/day12/sample{}", index + 1);
                    let caves = load_caves(file).unwrap();
                    assert_eq!(want, solve(&caves));
                }
            }
        }
    }
}

fn main() {
    let input = "tests/day12/input";
    let caves = day12::load_caves(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day12::part1::solve(&caves));
}
