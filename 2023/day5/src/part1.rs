use std::{ops::Range, str::FromStr};

struct MapLine {
    target_delta: i64,
    source_range: Range<i64>,
}

impl MapLine {
    fn try_apply(&self, source: i64) -> Option<i64> {
        self.source_range
            .contains(&source)
            .then(|| source + self.target_delta)
    }
}

impl FromStr for MapLine {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.splitn(3, ' ');
        let target_start: i64 = words
            .next()
            .ok_or("expected start of target range")?
            .parse()
            .or(Err("target range start should be a number"))?;
        let source_start: i64 = words
            .next()
            .ok_or("expected start of source range")?
            .parse()
            .or(Err("source range start should be a number"))?;
        let range_len: i64 = words
            .next()
            .ok_or("expected range length")?
            .parse()
            .or(Err("range length should be a number"))?;
        assert!(words.next().is_none());
        Ok(MapLine {
            target_delta: target_start - source_start,
            source_range: (source_start..source_start + range_len),
        })
    }
}

struct Map {
    lines: Vec<MapLine>,
}

impl Map {
    fn apply(&self, source: i64) -> i64 {
        for line in &self.lines {
            if let Some(target) = line.try_apply(source) {
                return target;
            }
        }
        source
    }
}

impl FromStr for Map {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Result<Vec<MapLine>, _> = s.lines().skip(1).map(|line| line.parse()).collect();
        Ok(Map { lines: lines? })
    }
}

fn parse_seeds(line: &str) -> Vec<i64> {
    let colon = line.find(':').expect("colon in seed line");
    line[colon + 1..]
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().expect("seed number"))
        .collect()
}

pub fn solve(text: &str) -> i64 {
    let mut paragraphs = text.split("\n\n");
    let seeds = parse_seeds(paragraphs.next().expect("seeds on first line"));
    let maps: Vec<Map> = paragraphs.map(|s| s.parse().expect("map")).collect();
    seeds
        .into_iter()
        .map(|seed| maps.iter().fold(seed, |source, map| map.apply(source)))
        .min()
        .expect("at least one seed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 35);
    }
}
