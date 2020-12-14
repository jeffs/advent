use super::instruction::Instruction;
use super::machine::Machine;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solve<P>(input: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut machine = Machine::new();
    for line in BufReader::new(File::open(input)?).lines() {
        machine.execute2(Instruction::parse(line?)?);
    }
    Ok(machine.sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample2() {
        let input = "tests/day14/sample2";
        assert_eq!(208, solve(input).unwrap());
    }
}
