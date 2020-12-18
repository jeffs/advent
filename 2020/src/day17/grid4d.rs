use super::cube::Cube;
use super::point4d::Point4d;
use crate::error::ParseError;
use std::collections::HashSet;
use std::ops::Index;
use std::str::FromStr;

/// An infinite set of cubes arranged contiguously in 3-space.
#[derive(Clone, Debug)]
pub struct Grid4d {
    active: HashSet<Point4d>,
}

impl Grid4d {
    fn count_neighbors(&self, point: Point4d) -> usize {
        point
            .neighbors()
            .filter(|&address| self[address].is_active())
            .count()
    }

    pub fn next(self) -> Grid4d {
        //  for each active cube
        //      consider the point and its neighbors
        //      if their "next" is active, add them to the next active set
        let mut seen = HashSet::new();
        let mut active = HashSet::new();
        for &p in self.active.iter() {
            let n = self.count_neighbors(p);
            if self[p].next(n).is_active() {
                active.insert(p);
            }
            for q in p.neighbors() {
                if seen.contains(&q) {
                    continue;
                }
                seen.insert(q);
                let n = self.count_neighbors(q);
                if self[q].next(n).is_active() {
                    active.insert(q);
                }
            }
        }
        Grid4d { active }
    }

    /// Returns the number of active cubes in this grid.
    pub fn population(&self) -> usize {
        self.active.len()
    }

    pub fn advance(mut self, time: usize) -> Grid4d {
        for _ in 0..time {
            self = self.next();
        }
        self
    }
}

impl Default for Grid4d {
    fn default() -> Grid4d {
        Grid4d {
            active: HashSet::new(),
        }
    }
}

impl Index<Point4d> for Grid4d {
    type Output = Cube;

    fn index(&self, point: Point4d) -> &Self::Output {
        if self.active.contains(&point) {
            &Cube::Active
        } else {
            &Cube::Inactive
        }
    }
}

impl FromStr for Grid4d {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s
            .lines()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();
        if lines.is_empty() {
            return Err(ParseError::new("empty grid"));
        }
        let dx = lines[0].len();
        if lines.iter().any(|s| s.len() != dx) {
            return Err(ParseError::new("jagged grid"));
        }
        let mut active = HashSet::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, cube) in line.chars().enumerate() {
                if Cube::parse(cube)?.is_active() {
                    active.insert(Point4d(x as isize, y as isize, 0, 0));
                }
            }
        }
        Ok(Grid4d { active })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample1() -> Grid4d {
        ".#.
         ..#
         ###"
        .parse()
        .unwrap()
    }

    #[test]
    fn count_neighbors() {
        let grid = sample1();
        assert_eq!(1, grid.count_neighbors(Point4d(0, 0, -1, 0)));
        assert_eq!(2, grid.count_neighbors(Point4d(1, 0, -1, 0)));
    }

    #[test]
    fn outer_space() {
        assert_eq!(Cube::Inactive, sample1()[Point4d(-1, 0, 0, 0)]);
    }

    #[test]
    #[ignore]
    fn simulate() {
        assert_eq!(848, sample1().advance(6).population());
    }
}
