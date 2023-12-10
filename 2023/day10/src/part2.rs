use std::collections::HashSet;

use crate::{grid::Grid, position::Position, tile::Tile};

pub fn solve(text: &str) -> usize {
    let grid = Grid::from_str(text);
    let start = grid.start();

    // As we proceed clockwise through the loop, mark anything on the left
    // "outside."  Then, from each outer square, infect its entire contiguous
    // region with outerness.  Finally, count all the outer squares, and
    // subtract from the total number of ground squares.

    let mut old = start;
    let mut new = grid
        .exits(start)
        .next()
        .expect("somewhere reachable from the start position");
    let mut outside = HashSet::<Position>::new();
    while new != start {
        let pos = grid
            .exits(new)
            .find(|&pos| pos != old)
            .expect("somewhere reachable from each reachable position");
        let dir = new.dir(pos);
        if let Some(left) = pos.go(dir.left()) {
            if let Some(Tile::Ground) = grid.at(left) {
                // dbg!(left);
                outside.insert(left);
            }
        }
        (old, new) = (new, pos);
    }
    grid.ground_len() - outside.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        for (text, want) in include_str!("sample2.txt").split("\n\n").zip([4, 8]) {
            //, 8, 10]) {
            assert_eq!(solve(text), want);
        }
    }
}
