use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::path::{Path, PathBuf};

// EmptyFile

#[derive(Debug)]
pub struct EmptyFile {
    path: PathBuf,
}

impl EmptyFile {
    pub fn new<P>(path: P) -> EmptyFile
    where
        P: AsRef<Path>,
    {
        EmptyFile {
            path: path.as_ref().to_owned(),
        }
    }
}

impl Display for EmptyFile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: empty file", self.path.display())
    }
}

impl Error for EmptyFile {}

// ParseError

#[derive(Debug)]
pub struct ParseError {
    path: PathBuf,
    what: String,
}

impl ParseError {
    pub fn new<P>(path: P, what: &str) -> ParseError
    where
        P: AsRef<Path>,
    {
        ParseError {
            path: path.as_ref().to_owned(),
            what: what.to_owned(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: parse error: {}", self.path.display(), self.what)
    }
}

impl Error for ParseError {}

// NoSolution

#[derive(Debug)]
pub struct NoSolution;

impl Display for NoSolution {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "no solution")
    }
}

impl Error for NoSolution {}
