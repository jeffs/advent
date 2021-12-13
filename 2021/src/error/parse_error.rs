use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::io;
use std::num::ParseIntError;
use std::path::Path;

#[derive(Debug)]
pub struct ParseError {
    what: String,
}

impl ParseError {
    pub fn new<S: ToString>(what: S) -> ParseError {
        ParseError {
            what: what.to_string(),
        }
    }

    pub fn at<P, S>(path: P, line: usize, what: S) -> ParseError
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        ParseError::new(format!(
            "{}:{}: {}",
            path.as_ref().display(),
            line,
            what.as_ref()
        ))
    }

    pub fn in_file<P, S>(path: P, what: S) -> ParseError
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        ParseError::new(format!("{}: {}", path.as_ref().display(), what.as_ref()))
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

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        Self::new(err.to_string())
    }
}
