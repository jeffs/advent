use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub enum LevelError {
    Missing,
    Parse(ParseIntError),
    Extra,
}

impl From<ParseIntError> for LevelError {
    fn from(value: ParseIntError) -> Self {
        LevelError::Parse(value)
    }
}

pub struct Report(
    /// Levels.
    pub Vec<u8>,
);

impl FromStr for Report {
    type Err = LevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels = s
            .trim()
            .split_ascii_whitespace()
            .map(|level| level.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Report(levels))
    }
}

pub struct Input(pub Vec<Report>);

impl FromStr for Input {
    type Err = LevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports = s
            .trim()
            .lines()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>();
        Ok(Input(reports?))
    }
}
