#![allow(dead_code, unused_imports, unused_variables)]

use super::instruction::Instruction;
use super::mask::Mask;
use super::memory::Sparse;
use super::value::Value;
use crate::error::{NoSolution, ParseError};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve<P>(input: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut lines = BufReader::new(File::open(input)?).lines();
    let mask = Mask::parse_line(lines.next().ok_or(NoSolution)??)?;
    println!("{:?}", mask);
    let memory = Sparse::new();
    for line in lines {
        let instruction = Instruction::parse(line?)?;
        println!("{:?}", instruction);
    }
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample1() {
        let input = "tests/day14/sample1";
        assert_eq!(165, solve(input).unwrap());
    }
}
