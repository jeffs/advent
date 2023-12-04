use std::collections::HashSet;

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split_ascii_whitespace()
        .map(|word| word.parse().expect("number"))
        .collect()
}

pub fn solve_line(line: &str) -> u32 {
    let colon = line.find(':').expect("colon");
    let (want, got) = line[colon + 1..].split_once("|").expect("pipe");
    let count = parse_numbers(want)
        .intersection(&parse_numbers(got))
        .count();
    1 << count >> 1
}

pub fn solve(text: &str) -> u32 {
    text.lines().map(solve_line).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_line_sample() {
        for (line, want) in include_str!("sample.txt").lines().zip([8, 2, 2, 1, 0, 0]) {
            assert_eq!(solve_line(line), want);
        }
    }

    #[test]
    fn solve_sample() {
        assert_eq!(solve(include_str!("sample.txt")), 13);
    }
}
