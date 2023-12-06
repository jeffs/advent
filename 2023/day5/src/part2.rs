use std::ops::Range;

use crate::map::Map;

fn parse_seed_ranges(line: &str) -> Vec<Range<i64>> {
    let colon = line.find(':').expect("colon in seed line");
    let numbers: Vec<i64> = line[colon + 1..]
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().expect("number in seed line"))
        .collect();
    numbers
        .chunks_exact(2)
        .map(|chunk| {
            let &[start, len] = chunk else {
                panic!("seed line should have an even number of values");
            };
            start..start + len
        })
        .collect()
}

pub fn solve(text: &str) -> i64 {
    let mut paragraphs = text.split("\n\n");
    let ranges = parse_seed_ranges(paragraphs.next().expect("seeds on first line"));
    paragraphs
        .map(|s| s.parse::<Map>().expect("map"))
        .fold(ranges, |sources, map| map.apply_ranges(sources))
        .into_iter()
        .map(|range| range.start)
        .min()
        .expect("at least one seed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 46);
    }
}
