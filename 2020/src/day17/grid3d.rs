use super::cube::Cube;
use super::point3d::Point3d;
use crate::error::ParseError;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug)]
pub struct Boundary {
    min: Point3d,
    max: Point3d,
}

/// An infinite set of cubes arranged contiguously in 3-space.
#[derive(Clone, Debug)]
pub struct Grid3d {
    active: HashSet<Point3d>,
}

impl Grid3d {
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
                min: Point3d(xn, yn, zn),
                max: Point3d(xp, yp, zp),
            }
        })
    }

    fn count_neighbors(&self, point: Point3d) -> usize {
        point
            .neighbors()
            .filter(|&address| self[address].is_active())
            .count()
    }

    pub fn next(self) -> Grid3d {
        //  for each active cube
        //      consider the point and its neighbors
        //      if their "next" is active, add them to the next active set
        let mut active = HashSet::new();
        for &p in self.active.iter() {
            let n = self.count_neighbors(p);
            if self[p].next(n).is_active() {
                active.insert(p);
            }
            for q in p.neighbors() {
                let n = self.count_neighbors(q);
                if self[q].next(n).is_active() {
                    active.insert(q);
                }
            }
        }
        Grid3d { active }
    }

    /// Returns the number of active cubes in this grid.
    pub fn population(&self) -> usize {
        self.active.len()
    }

    pub fn advance(mut self, time: usize) -> Grid3d {
        for _ in 0..time {
            self = self.next();
        }
        self
    }
}

impl Default for Grid3d {
    fn default() -> Grid3d {
        Grid3d {
            active: HashSet::new(),
        }
    }
}

impl Index<Point3d> for Grid3d {
    type Output = Cube;

    fn index(&self, point: Point3d) -> &Self::Output {
        if self.active.contains(&point) {
            &Cube::Active
        } else {
            &Cube::Inactive
        }
    }
}

impl FromStr for Grid3d {
    type Err = ParseError;

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
        let mut active = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, cube) in line.chars().enumerate() {
                if Cube::parse(cube)?.is_active() {
                    active.insert(Point3d(x as isize, y as isize, 0));
                }
            }
        }
        Ok(Grid3d { active })
    }
}

impl Display for Grid3d {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.bounds() {
            Some(Boundary { min, max }) => write!(f, "{}",
                ((min.2)..=max.2).map(|z| {
                ((min.1)..=max.1).map(|y| {
                ((min.0)..=max.0).map(|x| {
                    self[Point3d(x, y, z)].to_string()
                }).collect::<Vec<_>>().join("")
                }).collect::<Vec<_>>().join("\n")
                }).collect::<Vec<_>>().join("\n\n")),
            None => Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample1() -> Grid3d {
        ".#.
         ..#
         ###"
        .parse()
        .unwrap()
    }

    #[test]
    fn count_neighbors() {
        let grid = sample1();
        assert_eq!(1, grid.count_neighbors(Point3d(0, 0, -1)));
        assert_eq!(2, grid.count_neighbors(Point3d(1, 0, -1)));
    }

    #[test]
    fn read_after_write() {
        assert_eq!(".#.\n..#\n###", sample1().to_string());
    }

    #[test]
    fn outer_space() {
        assert_eq!(Cube::Inactive, sample1()[Point3d(-1, 0, 0)]);
    }

    #[test]
    fn simulate() {
        assert_eq!(112, sample1().advance(6).population());
    }
}
