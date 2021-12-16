use advent2021::{CardinalNeighbors as Neighbors, Point};
use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

fn load_heights<P>(input: P) -> Result<Vec<Vec<u32>>, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut heights = Vec::new();
    for line in BufReader::new(File::open(&input)?).lines() {
        heights.push(line?.bytes().map(|b| (b - b'0') as u32).collect());
    }
    Ok(heights)
}

fn is_low_point(heights: &[Vec<u32>], p: Point) -> bool {
    let height = heights[p.0][p.1];
    Neighbors::of(heights, p).all(|q| height < heights[q.0][q.1])
}

fn low_points(heights: &[Vec<u32>]) -> impl Iterator<Item = Point> + '_ {
    (0..heights.len())
        .flat_map(|i| (0..heights[i].len()).map(move |j| (i, j)))
        .filter(|&p| is_low_point(heights, p))
}

pub mod part1 {
    use super::*;

    pub fn solve(heights: &[Vec<u32>]) -> u32 {
        low_points(heights).map(|(i, j)| heights[i][j] + 1).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::super::load_heights;
        use super::*;

        #[test]
        fn test_solve() {
            let heights = load_heights("tests/day9/sample").unwrap();
            assert_eq!(15, solve(&heights));
        }
    }
}

pub mod part2 {
    use super::*;

    fn basin_size(heights: &[Vec<u32>], p: Point) -> usize {
        let mut basin: HashSet<Point> = HashSet::new();
        let mut queue = VecDeque::from([p]);
        while let Some(p) = queue.pop_front() {
            for q in Neighbors::of(heights, p) {
                let height = heights[q.0][q.1];
                if height < 9 && !basin.contains(&q) {
                    basin.insert(q);
                    queue.push_back(q);
                }
            }
        }
        basin.len()
    }

    fn solve_n(heights: &[Vec<u32>], n: usize) -> usize {
        let mut sizes: Vec<_> = low_points(heights)
            .map(|p| basin_size(heights, p))
            .collect();
        sizes.select_nth_unstable_by(n - 1, |a, b| b.cmp(a));
        sizes[0..n].iter().product()
    }

    pub fn solve(heights: &[Vec<u32>]) -> usize {
        solve_n(heights, 3)
    }

    #[cfg(test)]
    mod tests {
        use super::super::load_heights;
        use super::*;

        #[test]
        fn test_solve() {
            let got = solve(&load_heights("tests/day9/sample").unwrap());
            assert_eq!(1134, got);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_low_point() {
        let heights = load_heights("tests/day9/sample").unwrap();
        let lows = [(0, 1), (0, 9), (2, 2), (4, 6)];
        for i in 0..heights.len() {
            for j in 0..heights[i].len() {
                let p = (i, j);
                let want = lows.contains(&p);
                let got = is_low_point(&heights, p);
                assert_eq!(want, got);
            }
        }
    }
}

fn main() {
    let input = "tests/day9/input";
    let heights = load_heights("tests/day9/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&heights));
    println!("{}", part2::solve(&heights));
}
