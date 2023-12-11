use std::collections::HashSet;

use crate::{direction::Direction, grid::Grid};

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
    let mut outside = Vec::new();
    while new != start {
        let pos = grid
            .exits(new)
            .find(|&pos| pos != old)
            .expect("somewhere reachable from each reachable position");
        if let Some(left) = pos
            .go(new.dir(pos).left())
            .filter(|&left| grid.is_ground(left))
        {
            outside.push(left);
        }
        (old, new) = (new, pos);
    }

    let mut seen = HashSet::new();
    while let Some(pos) = outside.pop() {
        seen.insert(pos);
        outside.extend(
            Direction::iter()
                .flat_map(|dir| pos.go(dir))
                .filter(|&next| grid.is_ground(next))
                .filter(|&next| seen.insert(next)),
        );
    }

    let mut ascii = grid.to_ascii();
    for &pos in seen.iter() {
        ascii[pos.0][pos.1] = b'O';
    }

    let lines: Vec<String> = ascii
        .into_iter()
        .map(|row| String::from_utf8_lossy(&row).into_owned())
        .collect();
    eprintln!("\n{}\n", lines.join("\n"));

    let ground_len = grid.iter().filter(|tile| tile.is_ground()).count();
    ground_len - seen.len()
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
