use super::{ParseError, Position, Size, Spot};
use std::error::Error;
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

    fn count_neighbors(&self, pos: &Position) -> usize {
        let Position { row, column } = *pos;
        assert!(row < self.width);
        assert!(column < self.height);
        let (is_top, is_bottom, is_left, is_right) = (
            (row == 0) as u8,
            (row + 1 == self.height) as u8,
            (column == 0) as u8,
            (column + 1 == self.width) as u8,
        );
        match is_top << 3 + is_bottom << 2 + is_left << 1 + is_right {
            // 0b0000 => count_neighbors_middle(pos),
            // 0b0001 => count_neighbors_right(pos),
            // 0b0010 => count
            // 0b0011 => todo!(),
            // 0b0100 => todo!(),
            // 0b0101 => todo!(),
            // 0b0110 => todo!(),
            // 0b0111 => todo!(),
            // 0b1000 => todo!(),
            // 0b1001 => todo!(),
            // 0b1010 => todo!(),
            // 0b1011 => todo!(),
            // 0b1100 => todo!(),
            // 0b1101 => todo!(),
            // 0b1110 => todo!(),
            // 0b1111 => todo!(),
            _ => unreachable!(),
        }
    }

    fn get(&self, pos: &Position) -> Spot {
        return self.spots[pos.row * self.width + pos.column];
    }

    pub fn next_to(&self, out: &mut Grid) {
        assert_eq!(out.height, self.height);
        assert_eq!(out.width, self.width);
        assert!(self.height > 1);
        assert!(self.width > 1);
        out.spots.clear();
        for i in 0..self.height {
            for j in 0..self.width {
                let pos = Position { row: i, column: j };
                let old = self.get(&pos);
                out.spots.push(if let Spot::Floor = old {
                    old
                } else {
                    let n = self.count_neighbors(&pos);
                    match old {
                        Spot::Empty if n == 0 => Spot::Occupied,
                        Spot::Occupied if n > 3 => Spot::Empty,
                        _ => old,
                    }
                });
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
