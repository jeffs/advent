use advent2020::NoSolution;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn is_sum_of_any_pair(number: usize, past: &VecDeque<usize>) -> bool {
    for m in past {
        for n in past {
            if m != n && m + n == number {
                return true;
            }
        }
    }
    false
}

fn read_preamble<I>(lines: I, size: usize) -> Result<VecDeque<usize>>
where
    I: Iterator<Item = io::Result<String>>,
{
    let mut preamble = VecDeque::new();
    for line in lines.take(size) {
        preamble.push_back(line?.parse()?);
    }
    Ok(preamble)
}

/// Reads numbers from the specified input file, beginning with a preamble of
/// the specified memory length.
fn solve_part1<P>(input: P, memory: usize) -> Result<usize>
where
    P: AsRef<Path>,
{
    let mut lines = BufReader::new(File::open(input)?).lines();
    let mut past = read_preamble(&mut lines, memory)?;
    for line in lines {
        let number = line?.parse()?;
        if !is_sum_of_any_pair(number, &past) {
            return Ok(number);
        }
        past.pop_front();
        past.push_back(number);
    }
    Err(Box::new(NoSolution))
}

fn read_numbers<P>(input: P) -> Result<Vec<usize>>
where
    P: AsRef<Path>,
{
    let mut numbers = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        numbers.push(line?.parse()?);
    }
    Ok(numbers)
}

/// Returns the sum of the smallest and largest of the specified numbers.
fn sum_min_max(numbers: &[usize]) -> usize {
    let min = numbers.iter().min().unwrap();
    let max = numbers.iter().max().unwrap();
    return min + max;
}

/// Returns the sum of the first and last numbers of a contiguous subsequence
/// from the specified input file that adds up to the specified series.
fn solve_part2<P>(input: P, series: usize) -> Result<usize>
where
    P: AsRef<Path>,
{
    let numbers = read_numbers(input)?;
    let sums: Vec<usize> = numbers // sum of numbers up to each index, inclusive
        .iter()
        .scan(0, |sum, number| {
            *sum += number;
            Some(*sum)
        })
        .collect();
    for end in 1..sums.len() {
        if sums[end] == series {
            println!("HERE");
            return Ok(sum_min_max(&numbers[0..end]));
        }
        for begin in 0..end {
            if sums[end] - sums[begin] == series {
                return Ok(sum_min_max(&numbers[(begin + 1)..end]));
            }
        }
    }
    Err(Box::new(NoSolution))
}

fn main() {
    let input = "tests/day9/input";
    let answer1 = solve_part1(input, 25).unwrap();
    println!("{}", answer1);
    println!("{}", solve_part2(input, answer1).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "tests/day9/sample1";
        assert_eq!(127, solve_part1(input, 5).unwrap());
    }

    #[test]
    fn sample2() {
        let input = "tests/day9/sample2";
        assert_eq!(62, solve_part2(input, 127).unwrap());
    }
}
