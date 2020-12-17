use crate::error::ParseError;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cube {
    Active,   // Represented by '#' in input.
    Inactive, // Represented by '.' in input.
}

impl Cube {
    pub fn parse(c: char) -> Result<Cube, ParseError> {
        match c {
            '#' => Ok(Cube::Active),
            '.' => Ok(Cube::Inactive),
            _ => Err(ParseError::new(format!("bad cube: {}", c))),
        }
    }

    pub fn is_active(self) -> bool {
        self == Cube::Active
    }

    /// Returns the next state of a cube having this state currently, given n
    /// active neighboring cubes.
    #[allow(dead_code)]
    pub fn next(&self, n: usize) -> Cube {
        match (self, n) {
            (Cube::Active, 2) => Cube::Active,
            (_, 3) => Cube::Active,
            (_, _) => Cube::Inactive,
        }
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Cube::Active => write!(f, "#"),
            Cube::Inactive => write!(f, "."),
        }
    }
}
