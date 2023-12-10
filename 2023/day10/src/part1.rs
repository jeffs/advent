use crate::grid::Grid;

pub fn solve(text: &str) -> usize {
    let grid = Grid::from_str(text);
    let start = grid.start();

    let mut old = start;
    let mut new = grid
        .exits(start)
        .next()
        .expect("somewhere reachable from the start position");
    let mut len = 1; // We already took one step, from old to new.
    while new != start {
        let pos = grid
            .exits(new)
            .find(|&pos| pos != old)
            .expect("somewhere reachable from each reachable position");

        (old, new) = (new, pos);
        len += 1;
    }
    (len + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        for (text, want) in include_str!("samples.txt").split("\n\n").zip([4, 4, 8, 8]) {
            assert_eq!(solve(text), want);
        }
    }
}
