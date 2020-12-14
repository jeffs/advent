#![allow(dead_code, unused_imports, unused_variables)]

use super::address::Address;
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
pub struct Instruction {
    address: Address,
    value: Value,
}

impl Instruction {
    pub fn parse<S: AsRef<str>>(line: S) -> Result<Instruction, ParseError> {
        let line = line.as_ref();
        Ok(Instruction {
            address: parse_address(line)?,
            value: parse_value(line)?,
        })
    }
}
