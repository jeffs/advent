use advent2020::error::{NoSolution, ParseError};
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

#[derive(Debug)]
enum Instruction {
    Acc(i32),
    Jmp(isize),
    Nop(isize),
}

type Program = Vec<Instruction>;

// Indicates the final accumulator value of a program that either completed
// normally (halted), or tried to enter an infinite loop.
enum Termination {
    Halt(i32),
    Loop(i32),
}

fn load_program<P: AsRef<Path>>(input: P) -> Result<Program, Box<dyn Error>> {
    let mut program = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        let line = line?;
        let tokens: Vec<_> = line.split_whitespace().collect();
        let instruction = match &tokens[..] {
            ["acc", arg] => Instruction::Acc(arg.parse()?),
            ["jmp", arg] => Instruction::Jmp(arg.parse()?),
            ["nop", arg] => Instruction::Nop(arg.parse()?),
            _ => {
                let what = format!("bad instruction: {}", line);
                return Err(Box::new(ParseError::new(what)));
            }
        };
        program.push(instruction);
    }
    Ok(program)
}

fn add_offset(pc: usize, arg: isize) -> usize {
    if arg < 0 {
        pc - -arg as usize
    } else {
        pc + arg as usize
    }
}

fn execute(program: &[Instruction]) -> Termination {
    let mut acc = 0; // accumulator
    let mut pc = 0; // program counter
    let mut seen: HashSet<usize> = HashSet::new(); // instruction indexes
    while !(pc == program.len() || seen.contains(&pc)) {
        seen.insert(pc);
        match program[pc] {
            Instruction::Acc(arg) => {
                acc += arg;
                pc += 1;
            }
            Instruction::Jmp(arg) => pc = add_offset(pc, arg),
            Instruction::Nop(_) => pc += 1,
        }
    }
    if pc == program.len() {
        Termination::Halt(acc)
    } else {
        Termination::Loop(acc)
    }
}

fn solve_part1(program: &[Instruction]) -> Result<i32, NoSolution> {
    if let Termination::Loop(acc) = execute(program) {
        Ok(acc)
    } else {
        Err(NoSolution)
    }
}

fn solve_part2(mut program: Program) -> Result<i32, NoSolution> {
    for i in 0..program.len() {
        match program[i] {
            Instruction::Jmp(arg) => {
                program[i] = Instruction::Nop(arg);
                if let Termination::Halt(acc) = execute(&program) {
                    return Ok(acc);
                }
                program[i] = Instruction::Jmp(arg);
            }
            Instruction::Nop(arg) => {
                program[i] = Instruction::Jmp(arg);
                if let Termination::Halt(acc) = execute(&program) {
                    return Ok(acc);
                }
                program[i] = Instruction::Nop(arg);
            }
            _ => (),
        }
    }
    Err(NoSolution)
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
    let answer1 = solve_part1(&program).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(1);
    });
    println!("{}", answer1);
    let answer2 = solve_part2(program).unwrap_or_else(|err| {
        eprintln!("error: {}", err);
        std::process::exit(2);
    });
    println!("{}", answer2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample1() {
        let program = load_program("tests/day8/sample1").unwrap();
        assert_eq!(5, solve_part1(&program).unwrap());
    }
}
