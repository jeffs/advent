#![allow(dead_code)]
use super::cube::Cube;
use super::point::Point;
use crate::error::ParseError;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
pub struct Boundary {
    min: Point,
    max: Point,
}

/// An infinite set of cubes arranged contiguously in 3-space.
#[derive(Debug)]
pub struct Grid {
    active: HashSet<Point>,
}

impl Grid {
    /// Returns the lower and upper bounds of this grid, as points.  The min point's
    /// components are all at least as low (i.e., nearest -∞) as the lowest
    /// corresponding component of any active cube in this grid, and the max
    /// point's are at least as high (nearest ∞).  Returns None if this grid
    /// contains no active cubes.
    fn bounds(&self) -> Option<Boundary> {
        self.active.iter().map(|p| p.0).min().map(|xn| {
            let yn = self.active.iter().map(|p| p.1).min().unwrap();
            let zn = self.active.iter().map(|p| p.2).min().unwrap();
            let xp = self.active.iter().map(|p| p.0).max().unwrap();
            let yp = self.active.iter().map(|p| p.1).max().unwrap();
            let zp = self.active.iter().map(|p| p.2).max().unwrap();
            Boundary {
                min: Point(xn, yn, zn),
                max: Point(xp, yp, zp),
            }
        })
    }

    /// Returns a point whose components are all at least as low (i.e., as
    /// close to -∞) as the lowest corresponding component of any active cube
    /// in this grid.  Returns None if this grid contains no active cubes.
    pub fn lower_bound(&self) -> Option<Point> {
        match (
            self.active.iter().map(|p| p.0).min(),
            self.active.iter().map(|p| p.1).min(),
            self.active.iter().map(|p| p.2).min(),
        ) {
            (Some(x), Some(y), Some(z)) => Some(Point(x, y, z)),
            _ => None,
        }
    }

    /// Returns the number of active cubes in this grid.
    pub fn population(&self) -> usize {
        self.active.len()
    }
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            active: HashSet::new(),
        }
    }
}

impl Index<Point> for Grid {
    type Output = Cube;

    fn index(&self, point: Point) -> &Self::Output {
        if self.active.contains(&point) {
            &Cube::Active
        } else {
            &Cube::Inactive
        }
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
        let mut active: HashSet<Point> = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, cube) in line.chars().enumerate() {
                if Cube::parse(cube)?.is_active() {
                    active.insert(Point(x as isize, y as isize, 0));
                }
            }
        }
        Ok(Grid { active: active })
    }
}

impl Display for Grid {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.bounds() {
            Some(Boundary { min, max }) => write!(f, "{}",
                ((min.2)..=max.2).map(|z| {
                ((min.1)..=max.1).map(|y| {
                ((min.0)..=max.0).map(|x| {
                    self[Point(x, y, z)].to_string()
                }).collect::<Vec<_>>().join("")
                }).collect::<Vec<_>>().join("\n")
                }).collect::<Vec<_>>().join("")),
            None => Ok(())
        }
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
        assert_eq!(Cube::Inactive, grid[Point(-1, 0, 0)]);
    }
}
