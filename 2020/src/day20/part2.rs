use super::neighbor::NeighborSet;
use super::rotate::clockwise;
use super::tile::{Projection, Tile};
use crate::error::NoSolution;
use std::collections::{HashMap, HashSet};
use std::error::Error;

const MONSTER_NOISE: usize = 15; // number of '#' per monster

fn is_monster_at(image: &[Vec<u8>], i: usize, j: usize) -> bool {
    //                    #
    //  #    ##    ##    ###
    //   #  #  #  #  #  #
    [
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ]
    .iter()
    .all(|(di, dj)| {
        let (y, x) = (i + di, j + dj);
        y < image.len() && x < image[0].len() && image[y][x] == b'#'
    })
}

fn count_monsters(image: &[Vec<u8>]) -> usize {
    let mut count = 0;
    for i in 0..image.len() {
        for j in 0..image[0].len() {
            if is_monster_at(&image, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn count_monsters_transformed(image: &[Vec<u8>]) -> Result<usize, NoSolution> {
    let count = count_monsters(image);
    if count != 0 {
        return Ok(count);
    }
    let mut image = image.to_vec();
    for _ in 1..4 {
        image = clockwise(&image);
        let count = count_monsters(&image);
        if count != 0 {
            return Ok(count);
        }
    }
    image.reverse();
    let count = count_monsters(&image);
    if count != 0 {
        return Ok(count);
    }
    for _ in 1..4 {
        image = clockwise(&image);
        let count = count_monsters(&image);
        if count != 0 {
            return Ok(count);
        }
    }
    Err(NoSolution)
}

struct Solver<'a> {
    neighbors: HashMap<&'a Projection, NeighborSet<'a>>,
    side: usize, // the edge length of the square image
    image: Vec<&'a Projection>,
    used: HashSet<u64>, // tile IDs,
}

impl<'a> Solver<'a> {
    fn new(tiles: &'a [Tile], projections: &'a [Projection]) -> Solver<'a> {
        Solver {
            neighbors: NeighborSet::graph(projections),
            side: (tiles.len() as f64).sqrt() as usize, // image is square
            image: Vec::new(),
            used: HashSet::new(),
        }
    }

    fn candidates(&self) -> Vec<&'a Projection> {
        let (side, len) = (self.side, self.image.len());
        let (i, j) = (len / side, len % side);
        self.image
            .last()
            .map(|p| {
                let v = if j == 0 {
                    let above = self.image[(i - 1) * self.side];
                    &self.neighbors[above].downs
                } else {
                    &self.neighbors[p].rights
                };
                v.iter()
                    .cloned()
                    .filter(|q| !self.used.contains(&q.tile_id))
                    .collect()
            })
            .unwrap_or_else(|| self.neighbors.keys().cloned().collect())
    }

    fn recur(&mut self) -> Option<Vec<Vec<u8>>> {
        if self.image.len() == self.side * self.side {
            return Some(self.render());
        }
        let candidates = self.candidates();
        for p in candidates {
            self.image.push(&p);
            self.used.insert(p.tile_id);
            let result = self.recur();
            if result.is_some() {
                return result;
            }
            self.used.remove(&p.tile_id);
            self.image.pop();
        }
        None
    }

    fn render(&self) -> Vec<Vec<u8>> {
        const INTERIOR_SIDE: usize = 8;
        let side = self.side;
        let mut rendered: Vec<Vec<u8>> = vec![Vec::new(); side * INTERIOR_SIDE];
        for i in 0..side {
            for j in 0..side {
                let p = self.image[i * side + j];
                for k in 0..INTERIOR_SIDE {
                    rendered[i * INTERIOR_SIDE + k].extend(p.interior[k].iter());
                }
            }
        }
        rendered
    }

    #[allow(clippy::naive_bytecount)]
    fn solve(mut self) -> Result<usize, NoSolution> {
        let image = self.recur().ok_or(NoSolution)?;
        let count = count_monsters_transformed(&image)?;
        if count == 0 {
            Err(NoSolution)
        } else {
            let noise: usize = image
                .iter()
                .map(|row| row.iter().filter(|&&b| b == b'#').count())
                .sum();
            Ok(noise - count * MONSTER_NOISE)
        }
    }
}

pub fn solve(text: &str) -> Result<usize, Box<dyn Error>> {
    let tiles = Tile::parse_all(text)?;
    let projections = Projection::collect(&tiles);
    Ok(Solver::new(&tiles, &projections).solve()?)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn solve_sample1() {
        let input_path = "tests/day20/sample1";
        let text = fs::read_to_string(input_path).unwrap();
        assert_eq!(273, solve(&text).unwrap());
    }

    #[test]
    fn search() {
        let image: Vec<Vec<u8>> = "
            .####...#####..#...###..
            #####..#..#.#.####..#.#.
            .#.#...#.###...#.##.##..
            #.#.##.###.#.##.##.#####
            ..##.###.####..#.####.##
            ...#.#..##.##...#..#..##
            #.##.#..#.#..#..##.#.#..
            .###.##.....#...###.#...
            #.####.#.#....##.#..#.#.
            ##...#..#....#..#...####
            ..#.##...###..#.#####..#
            ....#.##.#.#####....#...
            ..##.##.###.....#.##..#.
            #...#...###..####....##.
            .#.##...#.##.#.#.###...#
            #.###.#..####...##..#...
            #.###...#.##...#.######.
            .###.###.#######..#####.
            ..##.#..#..#.#######.###
            #.#..##.########..#..##.
            #.#####..#.#...##..#....
            #....##..#.#########..##
            #...#.....#..##...###.##
            #..###....##.#...##.##.#"
            .trim()
            .lines()
            .map(|line| line.trim().bytes().collect())
            .collect();
        assert!(is_monster_at(&image, 2, 2));
        assert_eq!(2, count_monsters(&image));
    }
}
