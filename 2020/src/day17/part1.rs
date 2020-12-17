use super::grid3d::Grid3d;
use std::error::Error;
use std::fs;

pub fn solve(input_path: &str) -> Result<usize, Box<dyn Error>> {
    let grid: Grid3d = fs::read_to_string(input_path)?.parse()?;
    Ok(grid.advance(6).population())
}
