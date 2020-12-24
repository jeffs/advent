use super::direction::HexDirection;
use super::latlon::LatLon;
use std::collections::HashSet;

use std::convert::Infallible;
use std::str::FromStr;

pub struct Floor {
    black: HashSet<LatLon>,
}

impl Floor {
    pub fn count_black(&self) -> usize {
        self.black.len()
    }

    //  allocate a buffer for the next state
    //  for each of 100 days
    //      clear the buffer
    //      for each tile
    //          for the tile and each of its six neighbors
    //              count black adjacent tiles
    //              conditionally insert in the buffer
    //      swap state and buffer 
    pub fn day(self, _n: usize) -> Floor {
        todo!()
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

    #[test]
    fn sample1() {
        let text = fs::read_to_string("tests/day24/sample1").unwrap();
        let floor: Floor = text.parse().unwrap();
        assert_eq!(10, floor.count_black());
        assert_eq!(2208, floor.day(100).count_black());
    }
}
