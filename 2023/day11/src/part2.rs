use crate::grid::Grid;

pub fn solve(text: &str) -> usize {
    Grid::from_str(text).distance(1000000)
}
