use super::{Position, Size, Spot};
use crate::error::ParseError;
use std::cmp;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Grid {
    height: usize,
    width: usize,
    spots: Vec<Spot>,
}

impl Grid {
    fn at(&self, pos: Position) -> Spot {
        assert!(pos.row < self.height);
        assert!(pos.column < self.width);
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
        Ok(Grid {
            height,
            width,
            spots,
        })
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

    fn count_neighbors1(&self, pos: Position) -> usize {
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

    fn count_neighbors2_right(&self, pos: Position) -> usize {
        let row = pos.row;
        for column in (pos.column + 1)..self.width {
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2_up_right(&self, pos: Position) -> usize {
        let distance = cmp::min(pos.row, self.width - 1 - pos.column);
        for delta in 1..=distance {
            let row = pos.row - delta;
            let column = pos.column + delta;
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2_up(&self, pos: Position) -> usize {
        let column = pos.column;
        for row in (0..pos.row).rev() {
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2_up_left(&self, pos: Position) -> usize {
        let distance = cmp::min(pos.row, pos.column);
        for delta in 1..=distance {
            let row = pos.row - delta;
            let column = pos.column - delta;
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2_left(&self, pos: Position) -> usize {
        let row = pos.row;
        for column in (0..pos.column).rev() {
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2_down_left(&self, pos: Position) -> usize {
        let distance = cmp::min(self.height - 1 - pos.row, pos.column);
        for delta in 1..=distance {
            let row = pos.row + delta;
            let column = pos.column - delta;
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2_down(&self, pos: Position) -> usize {
        let column = pos.column;
        for row in (pos.row + 1)..self.height {
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2_down_right(&self, pos: Position) -> usize {
        let distance = cmp::min(self.height - 1 - pos.row, self.width - 1 - pos.column);
        for delta in 1..=distance {
            let row = pos.row + delta;
            let column = pos.column + delta;
            match self.at(Position { row, column }) {
                Spot::Floor => (),
                Spot::Empty => return 0,
                Spot::Occupied => return 1,
            }
        }
        0
    }

    fn count_neighbors2(&self, pos: Position) -> usize {
        self.count_neighbors2_right(pos)
            + self.count_neighbors2_up_right(pos)
            + self.count_neighbors2_up(pos)
            + self.count_neighbors2_up_left(pos)
            + self.count_neighbors2_left(pos)
            + self.count_neighbors2_down_left(pos)
            + self.count_neighbors2_down(pos)
            + self.count_neighbors2_down_right(pos)
    }

    pub fn next1(&self, out: &mut Grid) {
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
                    let count = self.count_neighbors1(pos);
                    out.spots.push(self.at(pos).next1(count));
                }
            }
        }
    }

    pub fn next2(&self, out: &mut Grid) {
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
                    let count = self.count_neighbors2(pos);
                    out.spots.push(self.at(pos).next2(count));
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
        for row in 0..self.height {
            for column in 0..self.width {
                let pos = Position { row, column };
                self.at(pos).fmt(f)?
            }
            writeln!(f)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn count_neighbors2_sample2a() {
        let grid = Grid::from_file("tests/day11/sample2a").unwrap();
        let pos = Position { row: 4, column: 3 };
        assert_eq!(1, grid.count_neighbors2_right(pos));
        assert_eq!(1, grid.count_neighbors2_up_right(pos));
        assert_eq!(1, grid.count_neighbors2_up(pos));
        assert_eq!(1, grid.count_neighbors2_up_left(pos));
        assert_eq!(1, grid.count_neighbors2_left(pos));
        assert_eq!(1, grid.count_neighbors2_down_left(pos));
        assert_eq!(1, grid.count_neighbors2_down(pos));
        assert_eq!(1, grid.count_neighbors2_down_right(pos));
        assert_eq!(8, grid.count_neighbors2(pos));
    }

    #[test]
    fn count_neighbors2_sample2b() {
        let grid = Grid::from_file("tests/day11/sample2b").unwrap();
        let pos = Position { row: 1, column: 1 };
        assert_eq!(0, grid.count_neighbors2_right(pos));
        assert_eq!(0, grid.count_neighbors2_up_right(pos));
        assert_eq!(0, grid.count_neighbors2_up(pos));
        assert_eq!(0, grid.count_neighbors2_up_left(pos));
        assert_eq!(0, grid.count_neighbors2_left(pos));
        assert_eq!(0, grid.count_neighbors2_down_left(pos));
        assert_eq!(0, grid.count_neighbors2_down(pos));
        assert_eq!(0, grid.count_neighbors2_down_right(pos));
        assert_eq!(0, grid.count_neighbors2(pos));
    }
}
