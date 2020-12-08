use std::collections::HashSet;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

#[derive(Debug)]
struct InstructionParseError {
    line: String,
}

impl Display for InstructionParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "bad instruction: {}", self.line)
    }
}

impl Error for InstructionParseError {}

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(isize),
    Nop,
}

type Program = Vec<Instruction>;

fn load_program<P: AsRef<Path>>(input: P) -> Result<Program, Box<dyn Error>> {
    let mut program = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_whitespace().collect();
        let instruction = match &tokens[..] {
            ["acc", arg] => Instruction::Acc(arg.parse()?),
            ["jmp", arg] => Instruction::Jmp(arg.parse()?),
            ["nop", _] => Instruction::Nop,
            _ => return Err(Box::new(InstructionParseError { line })),
        };
        program.push(instruction);
    }
    Ok(program)
}

fn solve_part_1(program: &Program) -> i32 {
    let mut acc = 0; // accumulator
    let mut pc = 0; // program counter
    let mut seen: HashSet<isize> = HashSet::new(); // instruction indexes
    while !seen.contains(&pc) {
        seen.insert(pc);
        match program[pc as usize] {
            Instruction::Acc(arg) => {
                acc += arg;
                pc += 1;
            }
            Instruction::Jmp(arg) => pc += arg,
            Instruction::Nop => pc += 1,
        }
    }
    acc
}

fn main() {
    let input = "tests/day8/input";
    let program = match load_program(input) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(3);
        }
    };
    println!("{}", solve_part_1(&program));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let program = load_program("tests/day8/sample1").unwrap();
        assert_eq!(5, solve_part_1(&program));
    }
}
