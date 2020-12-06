use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::mem;
use std::path::Path;

fn solve_part1<P>(input: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut sum = 0;
    let mut group = HashSet::new();
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        if line.is_empty() {
            sum += mem::take(&mut group).len();
        } else {
            group.extend(line.chars());
        }
    }
    Ok(if group.is_empty() {
        sum
    } else {
        sum + group.len()
    })
}

fn solve_part2<P>(input: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut sum = 0;
    let mut group = HashSet::new();
    let mut first = true;
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        if line.is_empty() {
            first = true;
            sum += mem::take(&mut group).len();
        } else if first {
            first = false;
            group.extend(line.chars());
        } else {
            let person: HashSet<char> = line.chars().collect();
            group = group.intersection(&person).cloned().collect();
        }
    }
    Ok(if group.is_empty() {
        sum
    } else {
        sum + group.len()
    })
}

fn main() {
    let input = "tests/day6/input";
    println!("{}", solve_part1(input).unwrap());
    println!("{}", solve_part2(input).unwrap());
}
