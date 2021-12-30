use crate::player::Player;
use advent2021::ParseError;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

pub struct Puzzle {
    starts: [usize; 2],
}

impl Puzzle {
    #[cfg(test)]
    pub fn new(start1: usize, start2: usize) -> Puzzle {
        Puzzle {
            starts: [start1, start2],
        }
    }

    pub fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let input = input.as_ref();
        let mut starts = [0; 2];
        let mut output = starts.iter_mut();
        for line in BufReader::new(File::open(input)?).lines() {
            let start = line?
                .rsplit_once(' ')
                .ok_or_else(|| ParseError::in_file(input, "bad player; failed to split"))?
                .1
                .parse()?;
            *output
                .next()
                .ok_or_else(|| ParseError::in_file(input, "not enough players"))? = start;
        }
        if output.next().is_some() {
            return Err(ParseError::in_file(input, "too many players"));
        }
        Ok(Puzzle { starts })
    }

    pub fn new_players(&self) -> [Player; 2] {
        [
            Player {
                position: self.starts[0],
                score: 0,
            },
            Player {
                position: self.starts[1],
                score: 0,
            },
        ]
    }
}
