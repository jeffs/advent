use advent2021::NoSolution;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

mod day3 {
    use super::*;

    pub fn load_numbers<P: AsRef<Path>>(input: P) -> Result<Vec<String>, io::Error> {
        let mut numbers = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            numbers.push(line?);
        }
        Ok(numbers)
    }

    pub mod part1 {
        use super::*;

        /// Returns the count of 1 digits in each position.
        pub fn popcount_columns(numbers: &[String], width: usize) -> Vec<usize> {
            let mut counts = vec![0; width];
            for number in numbers {
                for (column, digit) in number.bytes().enumerate() {
                    if digit == b'1' {
                        counts[column] += 1;
                    }
                }
            }
            counts
        }

        pub fn parse_gamma(counts: &[usize], min_count: usize) -> u32 {
            let mut gamma = 0;
            for &count in counts {
                gamma <<= 1;
                if count >= min_count {
                    gamma |= 1;
                }
            }
            gamma
        }

        pub fn solve(numbers: &[String]) -> Result<u32, NoSolution> {
            let width = numbers.first().ok_or(NoSolution)?.len();
            let counts = popcount_columns(numbers, width);
            let gamma = parse_gamma(&counts, numbers.len() / 2);
            let epsilon = !gamma & ((1 << width) - 1);
            Ok(gamma * epsilon)
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                assert_eq!(Ok(198), solve(&load_numbers("tests/day3/sample").unwrap()));
            }
        }
    }
}

fn main() {
    let input = "tests/day3/input";
    let numbers = day3::load_numbers(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    match day3::part1::solve(&numbers) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
}
