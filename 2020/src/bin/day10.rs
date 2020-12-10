use advent2020::EmptyFile;
use std::collections::HashMap;
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
    for line in BufReader::new(File::open(&input)?).lines() {
        joltages.push(line?.parse()?);
    }
    joltages.sort();
    if let Some(&last) = joltages.last() {
        joltages.push(last + 3);
        Ok(joltages)
    } else {
        Err(Box::new(EmptyFile::new(input)))
    }
}

fn solve_part1(adapters: Vec<u32>) -> usize {
    let deltas: Vec<u32> = adapters
        .iter()
        .scan(0, |x, &y| Some(y - mem::replace(x, y)))
        .collect();
    let count1 = deltas.iter().cloned().filter(|&d| d == 1).count();
    let count3 = deltas.iter().cloned().filter(|&d| d == 3).count();
    count1 * count3
}

fn take_kids<'a, I>(key: u32, tail: I) -> Vec<u32>
where
    I: IntoIterator<Item = &'a u32>,
{
    const MAX_DELTA: u32 = 3;
    tail.into_iter()
        .cloned()
        .take_while(|&joltage| joltage - key <= MAX_DELTA)
        .collect()
}

fn solve_part2(adapters: Vec<u32>) -> usize {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    graph.insert(0, take_kids(0, &adapters));
    for i in 0..adapters.len() {
        let key = adapters[i];
        graph.insert(key, take_kids(key, &adapters[(i + 1)..]));
    }
    use std::iter::FromIterator;
    println!("{:?}", std::collections::BTreeMap::from_iter(graph.iter()));
    0 // TODO
}

fn main() {
    let input = "tests/day10/input";
    let adapters = load_joltages(input).unwrap();
    println!("{}", solve_part1(adapters.clone()));
    println!("{}", solve_part2(adapters));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_sample1a() {
        let adapters = load_joltages("tests/day10/sample1a").unwrap();
        assert_eq!(35, solve_part1(adapters));
    }

    #[test]
    fn part1_sample1b() {
        let adapters = load_joltages("tests/day10/sample1b").unwrap();
        assert_eq!(220, solve_part1(adapters));
    }

    #[test]
    fn part2_sample1a() {
        let adapters = load_joltages("tests/day10/sample1a").unwrap();
        assert_eq!(8, solve_part2(adapters));
    }

    #[test]
    fn part2_sample1b() {
        let adapters = load_joltages("tests/day10/sample1b").unwrap();
        assert_eq!(19208, solve_part2(adapters));
    }
}
