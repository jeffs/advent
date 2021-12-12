use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

mod day12 {
    use super::*;

    type Cave = String;
    type Map = HashMap<Cave, HashSet<Cave>>;

    pub fn load_caves<P>(input: P) -> Result<Vec<Vec<u8>>, io::Error>
    where
        P: AsRef<Path>,
    {
        let mut result = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            result.push(line?.into_bytes());
        }
        Ok(result)
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
