use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day7 {
    use super::*;

    pub fn load_crabs<P: AsRef<Path>>(input: P) -> Result<Vec<i32>, Box<dyn Error>> {
        let mut crabs = Vec::new();
        for line in BufReader::new(File::open(&input)?).lines() {
            for field in line?.split(',') {
                crabs.push(field.parse()?);
            }
        }
        Ok(crabs)
    }

    pub mod part1 {
        pub fn solve(crabs: &[i32]) -> i32 {
            assert!(!crabs.is_empty());
            let lo = *crabs.iter().min().unwrap();
            let hi = *crabs.iter().max().unwrap();
            let fuel = |y: i32| crabs.iter().map(|&x| (y - x).abs()).sum();
            (lo..=hi).map(fuel).min().unwrap()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_crabs;
            use super::solve;

            #[test]
            fn test_solve() {
                assert_eq!(37, solve(&load_crabs("tests/day7/sample").unwrap()));
            }
        }
    }

    pub mod part2 {
        pub fn solve(crabs: &[i32]) -> i32 {
            assert!(!crabs.is_empty());
            let lo = *crabs.iter().min().unwrap();
            let hi = *crabs.iter().max().unwrap();
            let triangle = |d: i32| d * (d + 1) / 2;
            let fuel = |y: i32| crabs.iter().map(|&x| triangle((y - x).abs())).sum();
            (lo..=hi).map(fuel).min().unwrap()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_crabs;
            use super::solve;

            #[test]
            fn test_solve() {
                assert_eq!(168, solve(&load_crabs("tests/day7/sample").unwrap()));
            }
        }
    }
}

fn main() {
    let input = "tests/day7/input";
    let crabs = day7::load_crabs("tests/day7/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day7::part1::solve(&crabs));
    println!("{}", day7::part2::solve(&crabs));
}
