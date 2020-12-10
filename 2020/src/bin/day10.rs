use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

fn load_joltages<P>(input: P) -> Result<Vec<u32>>
where
    P: AsRef<Path>,
{
    let mut joltages = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        joltages.push(line?.parse()?);
    }
    Ok(joltages)
}

fn solve_part1(mut adapters: Vec<u32>) -> usize {
    adapters.sort();
    adapters.push(*adapters.last().unwrap() + 3);
    let deltas: Vec<u32> = adapters
        .iter()
        .scan(0, |i, &j| Some(j - mem::replace(i, j)))
        .collect();
    let count1 = deltas.iter().cloned().filter(|&d| d == 1).count();
    let count3 = deltas.iter().cloned().filter(|&d| d == 3).count();
    count1 * count3
}

fn main() {
    let input = "tests/day10/input";
    let adapters = load_joltages(input).unwrap();
    println!("{}", solve_part1(adapters));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1a() {
        let adapters = load_joltages("tests/day10/sample1a").unwrap();
        assert_eq!(35, solve_part1(adapters));
    }

    #[test]
    fn sample1b() {
        let adapters = load_joltages("tests/day10/sample1b").unwrap();
        assert_eq!(220, solve_part1(adapters));
    }
}
