use super::cube::Cube;
use super::point::Point;
use crate::error::ParseError;
use std::fmt::{self, Display, Formatter};
use std::ops::{Index, Range};
use std::str::FromStr;

/// The dimensions of a grid's internal storage block.
struct SpaceRange {
    x: Range<isize>,
    y: Range<isize>,
    z: Range<isize>,
}

impl SpaceRange {
    fn linearize(&self, point: Point) -> usize {
        let dx = self.x.len();
        let dy = self.y.len();
        let x = (point.x - self.x.start) as usize;
        let y = (point.y - self.y.start) as usize;
        let z = (point.z - self.z.start) as usize;
        z * dx * dy + y * dx + x
    }
}

/// An infinite set of cubes arranged contiguously in 3-space.
#[allow(dead_code)]
pub struct Grid {
    cubes: Vec<Cube>,
    range: SpaceRange,
}

impl Grid {
    pub fn population(&self) -> usize {
        self.cubes.iter().filter(|cube| cube.is_active()).count()
    }
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            cubes: Vec::new(),
            range: SpaceRange {
                x: 0..0,
                y: 0..0,
                z: 0..0,
            },
        }
    }
}

impl Index<Point> for Grid {
    type Output = Cube;

    fn index(&self, point: Point) -> &Self::Output {
        let index = self.range.linearize(point);
        self.cubes.get(index).unwrap_or(&Cube::Inactive)
    }
}

impl FromStr for Grid {
    type Err = ParseError;

    #[allow(unused_variables)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if lines.is_empty() {
            return Err(ParseError::new("empty grid".to_owned()));
        }
        let dx = lines[0].len();
        if lines.iter().any(|s| s.len() != dx) {
            return Err(ParseError::new("jagged grid".to_owned()));
        }
        let cubes: Result<Vec<Cube>, ParseError> = lines
            .iter()
            .flat_map(|line| line.chars())
            .map(Cube::parse)
            .collect();
        Ok(Grid {
            cubes: cubes?,
            range: SpaceRange {
                x: 0..dx as isize,
                y: 0..lines.len() as isize,
                z: 0..1,
            },
        })
    }
}

impl Display for Grid {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = self.range.z.clone().map(|z| {
                self.range.y.clone().map(|y| {
                self.range.x.clone().map(|x| {
                     self[Point { x, y, z }].to_string()
                }).collect::<Vec<_>>().join("")
                }).collect::<Vec<_>>().join("\n")
                }).collect::<Vec<_>>().join("");
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE1: &str = "
        .#.
        ..#
        ###
    ";

    #[test]
    fn read_after_write() {
        let grid: Grid = SAMPLE1.parse().unwrap();
        assert_eq!(".#.\n..#\n###", grid.to_string());
    }

    #[test]
    fn outer_space() {
        let grid: Grid = SAMPLE1.parse().unwrap();
        assert_eq!(Cube::Inactive, grid[Point { x: -1, y: 0, z: 0 }]);
    }
}
