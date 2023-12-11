use std::collections::HashSet;

use crate::{direction::Direction, grid::Grid, position::Position, tile::Tile};

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

    // Clear junk.
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            let pos = Position(i, j);
            if !main.contains(&pos) {
                grid.set(pos, Tile::Ground);
            }
        }
    }

    // To expand the start node, the grid needs to know what kind of pipe it is.
    grid.set(
        start,
        Tile::from_exits(grid.exits(start).map(|exit| start.dir(exit))),
    );

    let collapsed = grid.clone();

    let grid = grid.expand();
    let mut exterior = Vec::new();
    let (height, width) = (grid.height(), grid.width());
    exterior.extend(
        (0..width)
            .flat_map(|j| [Position(0, j), Position(height - 1, j)])
            .chain((1..height - 1).flat_map(|i| [Position(i, 0), Position(i, width - 1)]))
            .filter(|&pos| grid.is_ground(pos)),
    );

    let mut seen = HashSet::new();
    while let Some(pos) = exterior.pop() {
        seen.insert(pos);
        let cardinals @ [north, _, south, _] = [
            pos.go(Direction::North),
            pos.go(Direction::East),
            pos.go(Direction::South),
            pos.go(Direction::West),
        ];
        let mut neighbors: Vec<Position> = cardinals.iter().cloned().flatten().collect();
        if let Some(north) = north {
            neighbors.extend(north.go(Direction::East));
            neighbors.extend(north.go(Direction::West));
        }
        if let Some(south) = south {
            neighbors.extend(south.go(Direction::East));
            neighbors.extend(south.go(Direction::West));
        }
        for neighbor in neighbors {
            if grid.is_ground(neighbor) && seen.insert(neighbor) {
                exterior.push(neighbor);
            }
        }
    }

    let grid = collapsed;
    let seen: HashSet<Position> = seen
        .into_iter()
        .filter(|pos| pos.0 % 2 == 0 && pos.1 % 2 == 0)
        .map(|pos| Position(pos.0 / 2, pos.1 / 2))
        .collect();

    let interior: Vec<Position> = grid
        .enumerate()
        .filter(|(pos, tile)| tile.is_ground() && !seen.contains(pos))
        .map(|(pos, _)| pos)
        .collect();

    #[cfg(test)]
    {
        let mut ascii = grid.to_ascii();
        for &pos in seen.iter() {
            ascii[pos.0][pos.1] = b'O';
        }
        for &pos in interior.iter() {
            ascii[pos.0][pos.1] = b'I';
        }
        let lines: Vec<String> = ascii
            .into_iter()
            .map(|row| String::from_utf8_lossy(&row).into_owned())
            .collect();
        eprintln!("{}\n", lines.join("\n"));
    };

    interior.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let samples = include_str!("sample2.txt").split("\n\n");
        for (text, want) in samples.zip([4, 4, 8, 10]) {
            assert_eq!(solve(text), want);
        }
    }
}
