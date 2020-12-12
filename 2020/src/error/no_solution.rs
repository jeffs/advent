use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct NoSolution;

impl Display for NoSolution {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "no solution")
    }
}

impl Error for NoSolution {}
