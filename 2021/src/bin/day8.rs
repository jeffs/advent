#![allow(dead_code)]

use advent2021::ParseError;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;
use std::str::FromStr;

mod day8 {
    use super::*;

    fn split_into(source: &str, target: &mut [String]) {
        source
            .trim()
            .split(' ')
            .zip(target)
            .for_each(|(s, t)| *t = s.to_string());
    }

    /// Returns all 7! = 5040 permutations of signals a..=g.
    //fn make_permutations() -> Vec<[u8; 7]> {
    fn make_permutations() -> [[u8; 7]; 5040] {
        let mut buf = [b'a', b'b', b'c', b'd', b'e', b'f', b'g'];
        let mut permutations = Vec::new();
        permutations.reserve_exact(5040);
        generate(7, &mut buf, &mut permutations);
        let mut result = [[0; 7]; 5040];
        permutations
            .iter()
            .zip(&mut result)
            .for_each(|(s, t)| *t = *s);
        result
    }

    /// Heap's Algorithm: https://en.wikipedia.org/wiki/Heap's_algorithm
    fn generate(k: usize, a: &mut [u8; 7], output: &mut Vec<[u8; 7]>) {
        if k == 1 {
            output.push(*a);
        } else {
            generate(k - 1, a, output);
            for i in 0..(k - 1) {
                let j = if k % 2 == 0 { i } else { 0 };
                a.swap(j, k - 1);
                generate(k - 1, a, output);
            }
        }
    }

    #[derive(Debug, Default)]
    pub struct Entry {
        signals: [String; 10],
        digits: [String; 4],
    }

    impl FromStr for Entry {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut entry = Entry::default();
            let (signals, digits) = s
                .split_once('|')
                .ok_or_else(|| ParseError::new(format!("bad entry: {}", s)))?;
            split_into(signals, &mut entry.signals);
            split_into(digits, &mut entry.digits);
            Ok(entry)
        }
    }

    pub fn load_entries<P: AsRef<Path>>(input: P) -> Result<Vec<Entry>, Box<dyn Error>> {
        let mut entries = Vec::new();
        for line in BufReader::new(File::open(&input)?).lines() {
            entries.push(line?.parse()?);
        }
        Ok(entries)
    }

    pub mod part1 {
        use super::*;

        pub fn solve(entries: &[Entry]) -> usize {
            entries
                .into_iter()
                .flat_map(|entry| entry.digits.iter())
                .map(|digits| digits.len())
                .filter(|len| match len {
                    2 | 4 | 3 | 7 => true,
                    _ => false,
                })
                .count()
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                assert_eq!(26, solve(&load_entries("tests/day8/sample").unwrap()));
            }
        }
    }

    pub mod part2 {
        use super::*;

        pub fn solve(_entries: &[Entry]) -> usize {
            let permutations = make_permutations();
            println!("{:#?}", permutations);
            todo!()
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                assert_eq!(26, solve(&load_entries("tests/day8/sample").unwrap()));
            }
        }
    }
}

fn main() {
    let input = "tests/day8/input";
    let entries = day8::load_entries("tests/day8/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day8::part1::solve(&entries));
}
