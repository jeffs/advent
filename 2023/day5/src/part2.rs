use crate::map::Map;

fn parse_seeds(line: &str) -> Vec<i64> {
    let colon = line.find(':').expect("colon in seed line");
    let numbers: Vec<i64> = line[colon + 1..]
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().expect("number in seed line"))
        .collect();
    numbers
        .chunks_exact(2)
        .flat_map(|chunk| {
            let &[start, len] = chunk else {
                panic!("seed line should have an even number of values");
            };
            dbg!(start, len);
            start..start + len
        })
        .collect()
}

pub fn solve(text: &str) -> i64 {
    let mut paragraphs = text.split("\n\n");
    let seeds = parse_seeds(paragraphs.next().expect("seeds on first line"));
    let maps: Vec<Map> = paragraphs.map(|s| s.parse().expect("map")).collect();
    let n = seeds.len();
    let mut last_percent = 0;
    seeds
        .into_iter()
        .enumerate()
        .map(|(i, seed)| {
            let percent = i * 100 / n;
            if percent > last_percent {
                dbg!(percent);
                last_percent = percent;
            }
            maps.iter().fold(seed, |source, map| map.apply(source))
        })
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
