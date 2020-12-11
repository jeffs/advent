use std::error::Error;
use std::fmt::{Display, Formatter, Result};
use std::path::{Path, PathBuf};

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

#[derive(Debug)]
pub struct NoSolution;

impl Display for NoSolution {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "no solution")
    }
}

impl Error for NoSolution {}
