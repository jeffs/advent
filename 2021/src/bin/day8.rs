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
}

fn main() {
    let input = "tests/day8/input";
    let entries = day8::load_entries("tests/day8/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day8::part1::solve(&entries));
}
