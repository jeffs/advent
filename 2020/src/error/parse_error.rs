use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct ParseError {
    what: String,
}

impl ParseError {
    pub fn new(what: String) -> ParseError {
        ParseError { what }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.what)
    }
}

impl Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::new(err.to_string())
    }
}
