#![allow(dead_code)]
use crate::error::ParseError;
use std::collections::{HashMap, HashSet};

type TileId = u64;
type TileMap = HashMap<TileId, Tile>;

#[derive(Eq, Hash, PartialEq)]
enum Abutment {
    Top,
    Right,
    Bottom,
    Left,
}

struct Tile {
    top: String,
    right: String,
    bottom: String,
    left: String,
}

impl Tile {
    fn abuts(&self, other: &Tile) -> Option<Abutment> {
        if self.top == other.bottom
            || self.top == other.left
            || self.top.chars().rev().eq(other.top.chars())
            || self.top.chars().rev().eq(other.right.chars())
        {
            return Some(Abutment::Top);
        }
        if self.right == other.left
            || self.right == self.bottom
            || self.right.chars().rev().eq(other.right.chars())
            || self.right.chars().rev().eq(other.top.chars())
        {
            return Some(Abutment::Right);
        }
        if self.bottom == other.top
            || self.bottom == other.right
            || self.bottom.chars().rev().eq(other.bottom.chars())
            || self.bottom.chars().rev().eq(other.left.chars())
        {
            return Some(Abutment::Bottom);
        }
        if self.left == other.right
            || self.left == self.top
            || self.left.chars().rev().eq(other.left.chars())
            || self.left.chars().rev().eq(other.bottom.chars())
        {
            return Some(Abutment::Left);
        }
        None
    }

    fn is_corner<'a, I>(&self, others: I) -> bool
    where
        I: Iterator<Item = &'a Tile> + Clone,
    {
        use Abutment::*;
        let abutments: HashSet<_> = others.flat_map(|t| self.abuts(&t)).collect();
        abutments.len() == 2
            && (abutments.contains(&Top) || abutments.contains(&Bottom))
            && (abutments.contains(&Left) || abutments.contains(&Right))
    }
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
        let id: TileId = lines[0]
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
    let corners: Vec<_> = tiles
        .iter()
        .filter(|(_, tile)| tile.is_corner(tiles.values()))
        .map(|(id, _)| id)
        .collect();
    println!("{:?}", corners);
    Ok(corners.iter().cloned().product())
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
