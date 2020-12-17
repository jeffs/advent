use crate::error::ParseError;

#[derive(Clone, Copy)]
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
