use advent2021::{EmptyFile, ParseError};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day14 {
    use super::*;

    type Pair = [u8; 2];

    pub struct Puzzle {
        chain: Vec<u8>,
        rules: HashMap<Pair, u8>,
    }

    pub fn load_puzzle<P>(input: P) -> Result<Puzzle, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let input = input.as_ref();
        let mut lines = BufReader::new(File::open(input)?).lines();
        let chain = lines
            .next()
            .ok_or_else(|| EmptyFile::new(input))??
            .bytes()
            .collect();
        let line = lines
            .next()
            .ok_or_else(|| ParseError::in_file(input, "expected empty line"))??;
        if !line.is_empty() {
            let what = format!("expected empty line; got {}", line);
            return Err(Box::new(ParseError::in_file(input, what)));
        }
        let mut rules = HashMap::new();
        for line in lines {
            let line = line?;
            let bytes = line.as_bytes();
            if bytes.len() != 7 {
                let what = format!("expected rule; got {}", line);
                return Err(Box::new(ParseError::in_file(input, what)));
            }
            rules.insert([bytes[0], bytes[1]], bytes[6]);
        }
        Ok(Puzzle { chain, rules })
    }

    fn count_pairs(chain: &[u8]) -> HashMap<Pair, usize> {
        let mut counts = HashMap::new();
        for i in 0..(chain.len() - 1) {
            let key = [chain[i], chain[i + 1]];
            *counts.entry(key).or_default() += 1;
        }
        counts
    }

    fn update(counts: HashMap<Pair, usize>, rules: &HashMap<Pair, u8>) -> HashMap<Pair, usize> {
        let mut result = HashMap::new();
        for (key, count) in counts {
            if rules.contains_key(&key) {
                let (a, b, c) = (key[0], rules[&key], key[1]);
                *result.entry([a, b]).or_default() += count;
                *result.entry([b, c]).or_default() += count;
            } else {
                *result.entry(key).or_default() += count;
            }
        }
        result
    }

    fn solve(puzzle: &Puzzle, n: usize) -> usize {
        let mut counts = count_pairs(&puzzle.chain);
        for _ in 0..n {
            counts = update(counts, &puzzle.rules);
        }

        let mut c_counts: HashMap<u8, usize> = HashMap::new();
        for (k, v) in counts {
            *c_counts.entry(k[0]).or_default() += v;
        }
        *c_counts.entry(*puzzle.chain.last().unwrap()).or_default() += 1;
        c_counts.values().max().unwrap() - c_counts.values().min().unwrap()
    }

    pub mod part1 {
        use super::*;

        pub fn solve(puzzle: &Puzzle) -> usize {
            super::solve(puzzle, 10)
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_puzzle;
            use super::solve;

            #[test]
            fn test_solve() {
                let puzzle = load_puzzle("tests/day14/sample").unwrap();
                assert_eq!(1588, solve(&puzzle));
            }
        }
    }

    pub mod part2 {
        use super::*;

        pub fn solve(puzzle: &Puzzle) -> usize {
            super::solve(puzzle, 40)
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_puzzle;
            use super::solve;

            #[test]
            fn test_solve() {
                let puzzle = load_puzzle("tests/day14/sample").unwrap();
                assert_eq!(2188189693529, solve(&puzzle));
            }
        }
    }
}

fn main() {
    let input = "tests/day14/input";
    let puzzle = day14::load_puzzle(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day14::part1::solve(&puzzle));
    println!("{}", day14::part2::solve(&puzzle));
}
