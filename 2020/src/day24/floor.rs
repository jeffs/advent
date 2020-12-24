use super::direction::HexDirection;
use super::latlon::LatLon;
use std::collections::HashSet;

use std::convert::Infallible;
use std::mem;
use std::str::FromStr;

pub struct Floor {
    black: HashSet<LatLon>,
}

impl Floor {
    pub fn count_black(&self) -> usize {
        self.black.len()
    }

    pub fn next_is_black(&self, tile: LatLon) -> bool {
        let count = tile
            .neighbors()
            .filter(|adjacent| self.black.contains(adjacent))
            .count();
        count == 2 || (count == 1 && self.black.contains(&tile))
    }

    fn next_into(&self, next: &mut HashSet<LatLon>) {
        next.clear();
        for &tile in &self.black {
            if self.next_is_black(tile) {
                next.insert(tile);
            }
            for neighbor in tile.neighbors() {
                if self.next_is_black(neighbor) {
                    next.insert(neighbor);
                }
            }
        }
    }

    pub fn day(mut self, day: usize) -> Floor {
        let next = &mut HashSet::new();
        for _ in 0..day {
            self.next_into(next);
            mem::swap(&mut self.black, next);
        }
        self
    }
}

impl FromStr for Floor {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Floor, Self::Err> {
        let mut black = HashSet::new();
        let origin = LatLon(0, 0);
        for line in s.lines() {
            let tile = origin.hence(HexDirection::parse_line(line));
            if black.contains(&tile) {
                black.remove(&tile);
            } else {
                black.insert(tile);
            }
        }
        Ok(Floor { black })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    fn sample1() -> Floor {
        fs::read_to_string("tests/day24/sample1")
            .unwrap()
            .parse()
            .unwrap()
    }

    #[test]
    fn part1_sample1() {
        assert_eq!(10, sample1().count_black());
    }

    #[test]
    fn part2_sample1_day1() {
        assert_eq!(15, sample1().day(1).count_black());
    }

    #[test]
    fn part2_sample1_day100() {
        assert_eq!(2208, sample1().day(100).count_black());
    }
}
