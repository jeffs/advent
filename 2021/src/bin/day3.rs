use advent2021::{EmptyFile, NoSolution};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

/// Reads, parses, and returns a sequence of binary numbers.  Although the
/// numbers are returned as u32, they may have any number of bits (up to
/// 32).  The second element of the returned tuple is the actual width of
/// the numbers.
fn load_numbers<P: AsRef<Path>>(input: P) -> Result<(Vec<u32>, usize), Box<dyn Error>> {
    let mut lines = BufReader::new(File::open(&input)?).lines();
    let first = lines.next().ok_or_else(|| EmptyFile::new(input))??;
    let mut numbers = vec![u32::from_str_radix(&first, 2)?];
    for line in lines {
        numbers.push(u32::from_str_radix(&line?, 2)?);
    }
    Ok((numbers, first.len()))
}

/// Returns the count of 1 digits in each (big-endian) position.
fn popcount_columns(numbers: &[u32], width: usize) -> Vec<usize> {
    let mut counts = vec![0; width];
    for number in numbers {
        for (column, shift) in (0..width).rev().enumerate() {
            if number >> shift & 1 != 0 {
                counts[column] += 1
            }
        }
    }
    counts
}

pub mod part1 {
    use super::*;

    fn find_gamma(counts: &[usize], min_count: usize) -> u32 {
        let mut gamma = 0;
        for &count in counts {
            gamma <<= 1;
            if count >= min_count {
                gamma |= 1;
            }
        }
        gamma
    }

    fn find_gamma_and_epsilon(numbers: &[u32], width: usize) -> (u32, u32) {
        let counts = popcount_columns(numbers, width);
        let gamma = find_gamma(&counts, numbers.len() / 2);
        let epsilon = !gamma & ((1 << width) - 1);
        (gamma, epsilon)
    }

    pub fn solve(numbers: &[u32], width: usize) -> u32 {
        let (gamma, epsilon) = find_gamma_and_epsilon(numbers, width);
        gamma * epsilon
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            let (numbers, width) = load_numbers("tests/day3/sample").unwrap();
            assert_eq!(198, solve(&numbers, width));
        }
    }
}

pub mod part2 {
    use super::*;

    fn oxygen_generator_rating(numbers: &[u32], mut width: usize) -> u32 {
        let mut numbers = numbers.to_vec();
        while numbers.len() > 1 {
            let bit = (popcount_columns(&numbers, width)[0] * 2 >= numbers.len()) as u32;
            numbers.retain(|number| number >> (width - 1) & 1 == bit);
            width -= 1;
        }
        numbers[0]
    }

    fn co2_generator_rating(numbers: &[u32], mut width: usize) -> u32 {
        let mut numbers = numbers.to_vec();
        while numbers.len() > 1 {
            let bit = (popcount_columns(&numbers, width)[0] * 2 < numbers.len()) as u32;
            numbers.retain(|number| number >> (width - 1) & 1 == bit);
            width -= 1;
        }
        numbers[0]
    }

    pub fn solve(numbers: &[u32], width: usize) -> Result<u32, NoSolution> {
        let oxygen = oxygen_generator_rating(numbers, width);
        let co2 = co2_generator_rating(numbers, width);
        Ok(oxygen * co2)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_oxygen_generator_rating() {
            let (numbers, width) = load_numbers("tests/day3/sample").unwrap();
            assert_eq!(23, oxygen_generator_rating(&numbers, width));
        }

        #[test]
        fn test_solve() {
            let (numbers, width) = load_numbers("tests/day3/sample").unwrap();
            assert_eq!(Ok(230), solve(&numbers, width));
        }
    }
}

fn main() {
    let input = "tests/day3/input";
    let (numbers, width) = load_numbers(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&numbers, width));
    match part2::solve(&numbers, width) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
}
