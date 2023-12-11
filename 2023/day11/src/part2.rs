use crate::grid::Grid;

// 649863639480 is too high
pub fn solve(text: &str) -> usize {
    Grid::from_str(text).distance(1000000)
}
