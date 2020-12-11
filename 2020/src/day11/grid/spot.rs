use super::ParseError;

/// The state of some position in a Grid.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Spot {
    Floor,
    Empty,
    Occupied,
}

impl Spot {
    fn from_char(c: char) -> Result<Spot, ParseError> {
        match c {
            '.' => Ok(Spot::Floor),
            'L' => Ok(Spot::Empty),
            '#' => Ok(Spot::Occupied),
            _ => Err(ParseError::new(format!("{}: bad spot", c))),
        }
    }

    pub fn parse_line(line: &str) -> Result<Vec<Spot>, ParseError> {
        line.chars().map(Spot::from_char).collect()
    }
}
