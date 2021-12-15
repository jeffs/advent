use advent2021::ParseError;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day15 {
    use super::*;

    type Cave = Vec<Vec<u8>>;

    pub fn load_cave<P>(input: P) -> Result<Cave, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut cave = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            cave.push(line?.bytes().map(|b| b - b'0').collect());
        }
        Ok(cave)
    }

    pub mod part1 {
        use super::*;

        pub fn solve(cave: &Cave) -> usize {
            cave.iter()
                .flat_map(|row| row.iter().cloned())
                .map(|b| b as usize)
                .sum()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_cave;
            use super::solve;

            #[test]
            fn test_solve() {
                let cave = load_cave("tests/day15/sample").unwrap();
                assert_eq!(17, solve(&cave));
            }
        }
    }
}

fn main() {
    let input = "tests/day15/input";
    let cave = day15::load_cave(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day15::part1::solve(&cave));
}
