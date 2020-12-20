use crate::error::ParseError;
use std::collections::HashMap;

struct Tile {
    top: String,
    right: String,
    bottom: String,
    left: String,
}

fn collect_column(lines: &[&str], n: usize) -> String {
    lines.iter().flat_map(|line| line.chars().nth(n)).collect()
}

pub fn solve(text: &str) -> Result<u64, ParseError> {
    let mut tiles = HashMap::new();
    for tile in text.split("\n\n") {
        let lines: Vec<&str> = tile.lines().collect();
        if lines.len() < 3
            || lines[1].len() < 2
            || lines[2..].iter().any(|line| line.len() != lines[1].len())
        {
            return Err(ParseError::new("bad tile"));
        }
        let id: u64 = lines[0]
            .trim_end_matches(':')
            .split_whitespace()
            .last()
            .and_then(|id| id.parse().ok())
            .ok_or_else(|| ParseError::new("expected tile ID"))?;
        let edges = Tile {
            top: lines[1].to_owned(),
            right: collect_column(&lines[1..], lines[0].len() - 1),
            bottom: lines[lines.len() - 1].to_owned(),
            left: collect_column(&lines[1..], 0),
        };
        tiles.insert(id, edges);
    }
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let input_path = "tests/day20/sample1";
        let text = fs::read_to_string(input_path).unwrap();
        assert_eq!(20899048083289, solve(&text).unwrap());
    }
}
