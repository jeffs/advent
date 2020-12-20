mod projections;

use super::projection::Projection;
use crate::error::ParseError;
use std::str::FromStr;

fn collect_column(lines: &[&str], n: usize) -> String {
    lines.iter().flat_map(|line| line.chars().nth(n)).collect()
}

#[derive(Clone)]
pub struct Tile {
    id: u64,
    top: String,
    right: String,
    bottom: String,
    left: String,
}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() < 3
            || lines[1].len() < 2
            || lines[2..].iter().any(|line| line.len() != lines[1].len())
        {
            return Err(ParseError::new("bad tile"));
        }
        let id = lines[0]
            .trim_end_matches(':')
            .split_whitespace()
            .last()
            .and_then(|id| id.parse().ok())
            .ok_or_else(|| ParseError::new("expected tile ID"))?;
        Ok(Tile {
            id,
            top: lines[1].to_owned(),
            right: collect_column(&lines[1..], lines[0].len() - 1),
            bottom: lines[lines.len() - 1].to_owned(),
            left: collect_column(&lines[1..], 0),
        })
    }
}
