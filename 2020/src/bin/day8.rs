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
    Jmp(i32),
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

fn main() {
    let input = "tests/day8/input";
    let program = match load_program(input) {
        Ok(program) => program,
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(3);
        }
    };
    println!("{:?}", program);
}
