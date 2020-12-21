use super::neighbor::NeighborSet;
use super::tile::{Projection, Tile};
use crate::error::NoSolution;
use std::collections::{HashMap, HashSet};
use std::error::Error;

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
        let lines: Vec<String> = rendered
            .iter()
            .map(|v| String::from_utf8(v.clone()).unwrap())
            .collect();
        let text = lines.join("\n");
        println!("{}", text);
        rendered
    }

    fn solve(mut self) -> Result<usize, NoSolution> {
        // TODO:
        // * Flip and rotate until you see two sea monsters.
        // * Replace the sea monsters with O characters.
        Ok(self
            .recur()
            .ok_or(NoSolution)?
            .iter()
            .flat_map(|row| row.iter().filter(|&&b| b == b'#'))
            .count())
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
}
