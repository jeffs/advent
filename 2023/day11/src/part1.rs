use crate::grid::{Grid, Position};

pub fn solve(text: &str) -> usize {
    let grid = Grid::from_str(text);
    let grid = grid.expand();
    let galaxies: Vec<Position> = grid.galaxies().collect();

    galaxies
        .iter()
        .enumerate()
        .flat_map(|(index, p)| {
            galaxies[index + 1..]
                .iter()
                .map(|q| p.0.abs_diff(q.0) + p.1.abs_diff(q.1))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 374);
    }
}
