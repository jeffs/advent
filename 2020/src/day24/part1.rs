use super::latlon::LatLon;
use super::direction::HexDirection;
use std::collections::HashSet;

pub fn solve(text: &str) -> usize {
    let mut black: HashSet<LatLon> = HashSet::new();
    let origin = LatLon(0, 0);
    for line in text.lines() {
        let tile = origin.hence(HexDirection::parse_line(line));
        if black.contains(&tile) {
            black.remove(&tile);
        } else {
            black.insert(tile);
        }
    }
    black.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let text = fs::read_to_string("tests/day24/sample1").unwrap();
        assert_eq!(10, solve(&text));
    }
}
