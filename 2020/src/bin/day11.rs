use advent2020::{EmptyFile, ParseError};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct SpotParseError {
    what: char,
}

impl SpotParseError {
    pub fn new<P>(what: char) -> SpotParseError {
        SpotParseError { what }
    }
}

impl fmt::Display for SpotParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: bad spot", self.what)
    }
}

impl std::error::Error for SpotParseError {}

/// The state of some position in a Grid.
#[derive(Clone, Copy, Debug)]
enum Spot {
    Floor,
    Empty,
    Occupied,
}

impl Spot {
    fn from_char(c: char) -> Result<Spot, SpotParseError> {
        match c {
            '.' => Ok(Spot::Floor),
            'L' => Ok(Spot::Empty),
            '#' => Ok(Spot::Occupied),
            _ => Err(SpotParseError { what: c }),
        }
    }

    fn parse_line(line: &str) -> Result<Vec<Spot>, SpotParseError> {
        line.chars().map(Spot::from_char).collect()
    }
}

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    spots: Vec<Spot>,
}

impl Grid {
    fn from_file<P: AsRef<Path>>(input: P) -> DynResult<Grid> {
        let mut lines = BufReader::new(File::open(&input)?).lines();
        let first_line = lines.next().ok_or_else(|| EmptyFile::new(&input))??;
        if first_line.is_empty() {
            let what = "rows must not be empty";
            return Err(Box::new(ParseError::new(&input, what)));
        }
        let width = first_line.len();
        let mut height = 1;
        let mut spots = Spot::parse_line(&first_line)?;
        for line in lines {
            let line = line?;
            if line.len() != width {
                let what = "all rows must be the same length";
                return Err(Box::new(ParseError::new(&input, what)));
            }
            spots.extend(Spot::parse_line(&line)?.iter());
            height += 1;
        }
        Ok(Grid {
            height,
            width,
            spots,
        })
    }
}

fn solve_part1<P>(input: P) -> DynResult<usize>
where
    P: AsRef<Path>,
{
    let grid = Grid::from_file(input)?;
    println!("{:?}", grid);
    todo!()
}

fn main() {
    let input = "tests/day11/input";
    println!("{}", solve_part1(input).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve1_sample1() {
        let input = "tests/day11/sample1";
        assert_eq!(37, solve_part1(input).unwrap());
    }
}
