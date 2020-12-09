use advent2020::NoSolution;
use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn is_sum_of_any_pair(number: i32, past: &VecDeque<i32>) -> bool {
    for m in past {
        for n in past {
            if m != n && m + n == number {
                return true;
            }
        }
    }
    false
}

fn read_preamble<I>(lines: I, size: usize) -> Result<VecDeque<i32>>
where
    I: Iterator<Item = io::Result<String>>,
{
    let mut preamble = VecDeque::new();
    for line in lines.take(size) {
        preamble.push_back(line?.parse()?);
    }
    Ok(preamble)
}

// Reads numbers from the specified input file, beginning with a preamble of
// the specified memory length.
fn solve_part1<P>(input: P, memory: usize) -> Result<i32>
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

fn main() {
    let input = "tests/day9/input";
    println!("{}", solve_part1(input, 25).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let input = "tests/day9/sample1";
        assert_eq!(127, solve_part1(input, 5).unwrap());
    }
}
