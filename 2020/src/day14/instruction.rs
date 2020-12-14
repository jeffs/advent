#![allow(dead_code, unused_imports, unused_variables)]

use super::address::Address;
use super::mask::Mask;
use super::value::Value;
use crate::error::ParseError;

const PREFIX: &str = "mem[";

fn parse_address(line: &str) -> Result<Address, ParseError> {
    if !line.starts_with(PREFIX) {
        let what = format!("{}: bad instruction: expected '{}'", line, PREFIX);
        return Err(ParseError::new(what));
    }
    let end = line.find(']').ok_or_else(|| {
        let what = format!("{}: bad instruction: missing ']'", line);
        ParseError::new(what)
    })?;
    line[PREFIX.len()..end].parse()
}

fn parse_value(line: &str) -> Result<Value, ParseError> {
    let mut parts = line.splitn(3, ' ').skip(1);
    let text = match (parts.next(), parts.next()) {
        (Some("="), Some(text)) => Ok(text),
        _ => Err(ParseError::new(format!("{}: expected value", line))),
    }?;
    text.parse()
}

#[derive(Debug)]
pub enum Instruction {
    Assign(Address, Value),
    Mask(Mask),
}

impl Instruction {
    pub fn parse<S: AsRef<str>>(line: S) -> Result<Instruction, ParseError> {
        let line = line.as_ref();
        if line.starts_with("mask") {
            Ok(Instruction::Mask(Mask::parse_line(line)?))
        } else {
            Ok(Instruction::Assign(
                parse_address(line)?,
                parse_value(line)?,
            ))
        }
    }
}
