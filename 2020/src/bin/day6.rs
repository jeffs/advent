use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::mem;
use std::path::Path;

fn solve_part1<P>(input: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut groups = Vec::new();
    let mut group = HashSet::new();
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        if line.is_empty() {
            groups.push(mem::take(&mut group));
        } else {
            group.extend(line.chars());
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }
    Ok(groups.iter().map(|g| g.len()).sum())
}

fn solve_part2<P>(input: P) -> io::Result<usize>
where
    P: AsRef<Path>,
{
    let mut groups = Vec::new();
    let mut group = HashSet::new();
    let mut first = true;
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        if line.is_empty() {
            first = true;
            groups.push(mem::take(&mut group));
        } else if first {
            first = false;
            group.extend(line.chars());
        } else {
            let person: HashSet<char> = line.chars().collect(); 
            group = group.intersection(&person).cloned().collect();
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }
    Ok(groups.iter().map(|g| g.len()).sum())
}

fn main() {
    let input = "tests/day6/input";
    println!("{}", solve_part1(input).unwrap());
    println!("{}", solve_part2(input).unwrap());
}
