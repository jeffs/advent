#![allow(dead_code)]

use advent2021::{NoSolution, ParseError};
use std::collections::HashMap;
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

    /// Returns all 7! = 5040 permutations of signal patterns a..=g.
    fn make_permutations() -> [[u8; 7]; 5040] {
        let mut result = [[0; 7]; 5040];
        let mut buffer = [b'a', b'b', b'c', b'd', b'e', b'f', b'g'];
        generate(7, &mut buffer, &mut result.iter_mut());
        result
    }

    /// Heap's Algorithm: https://en.wikipedia.org/wiki/Heap's_algorithm
    fn generate(k: usize, a: &mut [u8; 7], output: &mut std::slice::IterMut<[u8; 7]>) {
        if k == 1 {
            *output.next().expect("buffer overrun") = *a;
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
        patterns: [String; 10],
        digits: [String; 4],
    }

    impl FromStr for Entry {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut entry = Entry::default();
            let (patterns, digits) = s
                .split_once('|')
                .ok_or_else(|| ParseError::new(format!("bad entry: {}", s)))?;
            split_into(patterns, &mut entry.patterns);
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

        const PATTERNS: [&'static str; 10] = [
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        ];

        fn position(pattern: &str) -> Option<i32> {
            PATTERNS
                .into_iter()
                .position(|element| element == pattern)
                .map(|index| index as i32)
        }

        fn to_key<I: Iterator<Item = u8>>(bytes: I) -> String {
            let mut bytes: Vec<_> = bytes.collect();
            bytes.sort();
            String::from_utf8_lossy(&bytes).to_string()
        }

        fn permute(permutation: [u8; 7], pattern: &str) -> String {
            let bytes = pattern
                .bytes()
                .map(|byte| permutation[(byte - b'a') as usize]);
            to_key(bytes)
        }

        fn find_values(permutation: [u8; 7], patterns: &[String]) -> Option<HashMap<String, i32>> {
            let mut values: HashMap<String, i32> = HashMap::new();
            for pattern in patterns {
                let permuted = permute(permutation, pattern);
                if let Some(value) = position(&permuted) {
                    values.insert(to_key(pattern.bytes()), value);
                } else {
                    return None;
                }
            }
            Some(values)
        }

        fn solve_entry(permutations: &[[u8; 7]; 5040], entry: &Entry) -> Result<i32, NoSolution> {
            for &permutation in permutations {
                if let Some(values) = find_values(permutation, &entry.patterns) {
                    let mut value = 0;
                    for digit in &entry.digits {
                        let key = to_key(digit.bytes());
                        value = value * 10 + values[&key];
                    }
                    return Ok(value);
                }
            }
            Err(NoSolution)
        }

        pub fn solve(entries: &[Entry]) -> Result<i32, NoSolution> {
            let permutations = make_permutations();
            let mut sum = 0;
            for entry in entries {
                sum += solve_entry(&permutations, entry)?;
            }
            Ok(sum)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_permute() {
                assert_eq!("abdfg", permute(*b"cfgabde", "cdfbe"));
            }

            #[test]
            fn test_find_values() {
                let permutation = *b"cfgabde";
                let patterns = [
                    "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb",
                    "cagedb", "ab",
                ]
                .map(|s| to_key(s.bytes()));
                let want: Option<HashMap<String, i32>> = Some(
                    patterns
                        .iter()
                        .cloned()
                        .zip([8, 5, 2, 3, 7, 9, 6, 4, 0, 1])
                        .collect(),
                );
                let got = find_values(permutation, &patterns);
                assert_eq!(want, got);
            }

            #[test]
            fn test_solve_entry() {
                let permutations = make_permutations();
                let entry = Entry {
                    patterns: [
                        "acedgfb", "cdfbe", "gcdfa", "fbcad", "dab", "cefabd", "cdfgeb", "eafb",
                        "cagedb", "ab",
                    ]
                    .map(|s| s.to_string()),
                    digits: ["cdfeb", "fcadb", "cdfeb", "cdbaf"].map(|s| s.to_string()),
                };
                assert_eq!(Ok(5353), solve_entry(&permutations, &entry));
            }

            #[test]
            fn test_solve() {
                let entries = load_entries("tests/day8/sample").unwrap();
                assert_eq!(Ok(61229), solve(&entries));
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
    match day8::part2::solve(&entries) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
}
