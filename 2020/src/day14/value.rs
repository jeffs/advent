#![allow(dead_code, unused_variables)]
use crate::error::ParseError;
use std::str::FromStr;

const MAX: usize = (1 << 36) - 1;

#[derive(Clone, Copy, Debug, Default)]
pub struct Value(usize);

impl From<usize> for Value {
    fn from(u: usize) -> Self {
        Value(u)
    }
}

impl From<Value> for usize {
    fn from(v: Value) -> Self {
        v.0
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Value, Self::Err> {
        let value: usize = s.parse()?;
        if MAX < value {
            let what = format!("{}: value is too large", value);
            Err(ParseError::new(what))
        } else {
            Ok(Value(value))
        }
    }
}
