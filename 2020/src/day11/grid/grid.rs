use super::{ParseError, Position, Size, Spot};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    height: usize,
    width: usize,
    spots: Vec<Spot>,
}

impl Grid {
    fn at(&self, pos: Position) -> Spot {
        self.spots[pos.row * self.width + pos.column]
    }

    pub fn from_file<P>(input: P) -> Result<Grid, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let mut lines = BufReader::new(File::open(&input)?).lines();
        let first_line = match lines.next() {
            Some(line) => line?,
            None => todo!("support empty grids"),
        };
        if first_line.is_empty() {
            let what = format!("{}: empty row", input.as_ref().display());
            return Err(Box::new(ParseError::new(what)));
        }
        let width = first_line.len();
        let mut height = 1;
        let mut spots = Spot::parse_line(&first_line)?;
        for line in lines {
            let line = line?;
            if line.len() != width {
                let what = format!("{}:{}: jagged rows", input.as_ref().display(), height);
                return Err(Box::new(ParseError::new(what)));
            }
            spots.extend(Spot::parse_line(&line)?.iter());
            height += 1;
        }
        if height < 2 || width < 2 {
            todo!("support single-row and single-column grids")
        } else {
            Ok(Grid {
                height,
                width,
                spots,
            })
        }
    }

    pub fn with_size(size: Size) -> Grid {
        if size.height < 2 || size.width < 2 {
            todo!("support single-row and single-column grids")
        } else {
            Grid {
                height: size.height,
                width: size.width,
                spots: vec![Spot::Floor; size.area()],
            }
        }
    }

    fn count_neighbors(&self, pos: Position) -> usize {
        // i and j are 0-based indexes into the 3x3 grid around pos.
        let mut count = 0;
        let Position { row, column } = pos;
        for i in 0..3 {
            for j in 0..3 {
                if (i, j) != (1, 1)
                    && row + i > 0
                    && column + j > 0
                    && row + i - 1 < self.height
                    && column + j - 1 < self.width
                    && self.at(Position {
                        row: row + i - 1,
                        column: column + j - 1,
                    }) == Spot::Occupied
                {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn next_buf(&self, out: &mut Grid) {
        assert_eq!(out.height, self.height);
        assert_eq!(out.width, self.width);
        assert!(self.height > 1);
        assert!(self.width > 1);
        out.spots.clear();
        for row in 0..self.height {
            for column in 0..self.width {
                let pos = Position { row, column };
                let old = self.at(pos);
                if old == Spot::Floor {
                    out.spots.push(old);
                } else {
                    let count = self.count_neighbors(pos);
                    out.spots.push(self.at(pos).next(count));
                }
            }
        }
    }

    pub fn pop_count(&self) -> usize {
        self.spots.iter().filter(|&&s| s == Spot::Occupied).count()
    }

    pub fn size(&self) -> Size {
        Size {
            height: self.height,
            width: self.width,
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for row in 0..self.height {
            for column in 0..self.width {
                let pos = Position { row, column };
                out.push(match self.at(pos) {
                    Spot::Floor => '.',
                    Spot::Empty => 'L',
                    Spot::Occupied => '#',
                });
            }
            out.push('\n');
        }
        write!(f, "{}", out)
    }
}
