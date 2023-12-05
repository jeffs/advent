use crate::map::Map;

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
