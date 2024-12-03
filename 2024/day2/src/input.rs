use std::{num::ParseIntError, str::FromStr};

pub type Level = u8;

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

#[derive(Clone)]
pub struct Report(
    /// Levels.
    pub Vec<Level>,
);

impl Report {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn remove(&mut self, index: usize) {
        self.0.remove(index);
    }
}

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

impl Input {
    #[cfg(test)]
    pub fn sample() -> Input {
        "7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9
        "
        .parse()
        .expect("sample input")
    }

    pub fn reports(&self) -> impl Iterator<Item = &Report> {
        self.0.iter()
    }
}

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
