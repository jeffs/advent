use std::collections::HashSet;

use crate::{grid::Grid, tile::Tile};

pub fn solve(text: &str) -> usize {
    // Remove junk pipe.
    // Expand the grid, so passages become ground.
    // From border ground, infect all exterior.  Infection my be diagonal.
    // Collapse the grid, removing all expanded cells.
    // Count uninfected ground cells.

    let mut grid = Grid::from_str(text);
    let start = grid.start();

    let mut main = HashSet::from([start]);
    let mut old = start;
    let mut new = grid
        .exits(start)
        .next()
        .expect("somewhere reachable from the start position");

    while new != start {
        main.insert(new);
        let pos = grid
            .exits(new)
            .find(|&pos| pos != old)
            .expect("somewhere reachable from each reachable position");
        (old, new) = (new, pos);
    }

    // To expand the start node, the grid needs to know what kind of pipe it is.
    grid.set(
        start,
        Tile::from_exits(grid.exits(start).map(|exit| start.dir(exit))),
    );

    let grid = grid.expand();

    let ascii = grid.to_ascii();
    // for &pos in seen.iter() {
    //     ascii[pos.0][pos.1] = b'O';
    // }

    let lines: Vec<String> = ascii
        .into_iter()
        .map(|row| String::from_utf8_lossy(&row).into_owned())
        .collect();
    eprintln!("\n{}\n", lines.join("\n"));

    4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let samples = include_str!("sample2.txt").split("\n\n");
        for (text, want) in samples.zip([4, 8, 10]) {
            assert_eq!(solve(text), want);
        }
    }
}
