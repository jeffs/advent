#![allow(dead_code, unused_variables)]

use std::str::FromStr;

const NEEDLE: &[u8; 4] = b"XMAS";

#[derive(Debug)]
pub enum Error {
    NonRectangularInput,
}

pub type Result<T> = std::result::Result<T, Error>;

fn is_valid(rows: &[Vec<u8>]) -> bool {
    rows.first()
        .map(Vec::len)
        .filter(|&width| width != 0) // Disallow empty rows.
        .is_some_and(|width| rows.iter().skip(1).all(|row| row.len() == width))
}

fn width(rows: &[Vec<u8>]) -> usize {
    rows.first().map(Vec::len).unwrap_or_default()
}

struct GridPoint<'a> {
    rows: &'a [Vec<u8>],
    row_index: usize,
    column_index: usize,
}

impl GridPoint<'_> {
    fn matches_down(&self) -> bool {
        self.row_index + NEEDLE.len() <= self.rows.len()
            && NEEDLE
                .iter()
                .enumerate()
                .all(|(dj, &b)| self.rows[self.row_index + dj][self.column_index] == b)
    }

    fn matches_left(&self) -> bool {
        let end = self.column_index + 1;
        end >= NEEDLE.len()
            && self.rows[self.row_index][end - NEEDLE.len()..end]
                .iter()
                .rev()
                .zip(NEEDLE)
                .all(|(b, c)| b == c)
    }

    fn matches_right(&self) -> bool {
        self.rows[self.row_index][self.column_index..].starts_with(NEEDLE)
    }

    fn matches_up(&self) -> bool {
        self.row_index >= NEEDLE.len()
            && NEEDLE
                .iter()
                .enumerate()
                .all(|(dj, &b)| self.rows[self.row_index - dj][self.column_index] == b)
    }
}

struct GridRow<'a> {
    rows: &'a [Vec<u8>],
    row_index: usize,
}

impl GridRow<'_> {
    fn column(&self, column_index: usize) -> GridPoint {
        GridPoint {
            rows: self.rows,
            row_index: self.row_index,
            column_index,
        }
    }

    fn points(&self) -> impl Iterator<Item = GridPoint> {
        (0..width(self.rows)).map(|j| self.column(j))
    }
}

struct Grid {
    rows: Vec<Vec<u8>>,
}

impl Grid {
    fn from_rows(rows: Vec<Vec<u8>>) -> Result<Grid> {
        is_valid(&rows)
            .then_some(Grid { rows })
            .ok_or(Error::NonRectangularInput)
    }

    fn row(&self, row_index: usize) -> GridRow {
        GridRow {
            rows: &self.rows,
            row_index,
        }
    }

    fn rows(&self) -> impl Iterator<Item = GridRow> {
        (0..self.rows.first().map(Vec::len).unwrap_or_default()).map(|i| self.row(i))
    }
}

impl FromStr for Grid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Grid::from_rows(
            s.trim()
                .lines()
                .map(|line| line.trim().as_bytes().to_vec())
                .collect(),
        )
    }
}

pub fn solve(input: &str) -> Result<usize> {
    let grid: Grid = input.parse()?;
    let mut sum = 0;
    for row in grid.rows() {
        for point in row.points() {
            sum += [
                point.matches_right(),
                point.matches_up(),
                point.matches_left(),
                point.matches_down(),
            ]
            .into_iter()
            .filter(|&b| b)
            .count()
        }
    }
    Ok(sum) // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "
        MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX    
    ";

    #[test]
    fn test_solve() -> Result<()> {
        assert_eq!(solve(SAMPLE)?, 8);
        Ok(())
    }
}
