use crate::error::ParseError;

pub enum Instruction {
    North { distance: usize },
    South { distance: usize },
    East { distance: usize },
    West { distance: usize },
    Left { degrees: usize },
    Right { degrees: usize },
    Forward { distance: usize },
}

impl Instruction {
    pub fn parse(mut line: String) -> Result<Instruction, ParseError> {
        if line.is_empty() {
            return Err(ParseError::new("empty instruction".to_owned()));
        }
        let action = line.as_bytes()[0];
        let value: usize = line.split_off(1).parse()?;
        Ok(match action {
            b'N' => Instruction::North { distance: value },
            b'E' => Instruction::East { distance: value },
            b'S' => Instruction::South { distance: value },
            b'W' => Instruction::West { distance: value },
            b'L' => Instruction::Left { degrees: value },
            b'R' => Instruction::Right { degrees: value },
            b'F' => Instruction::Forward { distance: value },
            _ => return Err(ParseError::new(format!("{}: bad action", action))),
        })
    }
}
