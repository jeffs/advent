fn parse_seeds(line: &str) -> Vec<i32> {
    let colon = line.find(':').expect("colon in seed line");
    line[colon + 1..]
        .split_ascii_whitespace()
        .map(|s| s.parse::<i32>().expect("seed number"))
        .collect()
}

pub fn solve(text: &str) -> i32 {
    let mut paragraphs = text.split("\n\n");
    let seeds = parse_seeds(paragraphs.next().expect("seeds on first line"));
    seeds.into_iter().min().expect("at least one seed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 35);
    }
}
