use advent2022::{BoxedError, StaticError};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;
use std::process::exit;

pub mod part1 {
    use super::*;

    fn item_to_index(item: u8) -> Result<usize, BoxedError> {
        let index = match item {
            b'a'..=b'z' => item - b'a',
            b'A'..=b'Z' => item - b'A' + 26,
            _ => return Err(StaticError::boxed("bad item")),
        };
        Ok(index as usize)
    }

    pub fn solve(path: impl AsRef<Path>) -> Result<u64, BoxedError> {
        let mut sum = 0;
        let file = File::open(path)?;
        for line in BufReader::new(file).lines() {
            let line = line?;
            let (first_compartment, second_compartment) = line.as_bytes().split_at(line.len() / 2);
            let second_compartment: HashSet<u8> = second_compartment.iter().cloned().collect();
            let mut misplaced = [false; 52];
            for &item in first_compartment {
                if second_compartment.contains(&item) {
                    misplaced[item_to_index(item)?] = true;
                }
            }
            let row_sum: u64 = misplaced
                .iter()
                .enumerate()
                .flat_map(|(index, is_misplaced)| {
                    let priority = index as u64 + 1;
                    is_misplaced.then_some(priority)
                })
                .sum();
            sum += row_sum;
        }
        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let answer = solve("tests/day3/sample").expect("sample should have an answer");
            assert_eq!(157, answer);
        }

        #[test]
        fn test_solve_input() {
            let answer = solve("tests/day3/input").expect("input should have an answer");
            assert_eq!(7597, answer);
        }
    }
}

fn main() {
    for solve in [part1::solve] {
        let answer = solve("tests/day3/input").unwrap_or_else(|err| {
            eprintln!("error: {err}");
            exit(1);
        });
        println!("{answer}");
    }
}
