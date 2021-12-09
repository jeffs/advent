use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day9 {
    use super::*;

    struct Neighbors {
        places: [(usize, usize); 4],
        index: usize,
    }

    #[rustfmt::skip]
    impl Neighbors {
        fn of(heights: &[Vec<u32>], i: usize, j: usize) -> Self {
            assert!(i < heights.len() && j < heights[0].len());
            let (m, n) = (heights.len(), heights[0].len());
            let (y, x) = (m - 1, n - 1);
            let (mut places, mut index) = ([(0, 0); 4], 4);
            if j > 0 { index -= 1; places[index] = (i, j - 1); } // West
            if i < y { index -= 1; places[index] = (i + 1, j); } // South
            if j < x { index -= 1; places[index] = (i, j + 1); } // East
            if i > 0 { index -= 1; places[index] = (i - 1, j); } // North
            Neighbors { places, index }
        }
    }

    impl Iterator for Neighbors {
        type Item = (usize, usize);

        fn next(&mut self) -> Option<Self::Item> {
            if let Some(&(i, j)) = self.places.get(self.index) {
                self.index += 1;
                Some((i, j))
            } else {
                None
            }
        }
    }

    pub fn load_heights<P: AsRef<Path>>(input: P) -> Result<Vec<Vec<u32>>, Box<dyn Error>> {
        let mut heights = Vec::new();
        for line in BufReader::new(File::open(&input)?).lines() {
            heights.push(line?.bytes().map(|b| (b - b'0') as u32).collect());
        }
        Ok(heights)
    }

    #[rustfmt::skip]
    fn is_low_point(heights: &[Vec<u32>], i: usize, j: usize) -> bool {
        assert!(i < heights.len() && j < heights[0].len());
        let (m, n, h) = (heights.len(), heights[0].len(), heights[i][j]);
        let (y, x) = (m - 1, n - 1);
        (i == 0 || h < heights[i - 1][j]) &&
        (i == y || h < heights[i + 1][j]) &&
        (j == 0 || h < heights[i][j - 1]) &&
        (j == x || h < heights[i][j + 1])
    }

    fn low_points(heights: &[Vec<u32>]) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..heights.len())
            .flat_map(|i| (0..heights[i].len()).map(move |j| (i, j)))
            .filter(|&(i, j)| is_low_point(heights, i, j))
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
                assert_eq!(15, solve(&load_heights("tests/day9/sample").unwrap()));
            }
        }
    }

    pub mod part2 {
        use super::*;

        fn basin_size(heights: &[Vec<u32>], i: usize, j: usize) -> usize {
            let mut basin: HashSet<(usize, usize)> = HashSet::new();
            let mut queue = VecDeque::from([(i, j)]);
            while let Some((i, j)) = queue.pop_front() {
                for neighbor in Neighbors::of(heights, i, j) {
                    let height = heights[neighbor.0][neighbor.1];
                    if height < 9 && !basin.contains(&neighbor) {
                        basin.insert(neighbor);
                        queue.push_back(neighbor);
                    }
                }
            }
            basin.len()
        }

        pub fn solve_nth(heights: &[Vec<u32>], n: usize) -> usize {
            let mut sizes: Vec<_> = low_points(heights)
                .map(|(i, j)| basin_size(heights, i, j))
                .collect();
            let (smaller, nth, _larger) = sizes.select_nth_unstable_by(n, |a, b| b.cmp(a));
            smaller.iter().cloned().product::<usize>() * *nth
        }

        pub fn solve(heights: &[Vec<u32>]) -> usize {
            solve_nth(heights, 2)
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_heights;
            use super::*;

            #[test]
            fn test_solve() {
                assert_eq!(1134, solve(&load_heights("tests/day9/sample").unwrap()));
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_low_point() {
            let heights = load_heights("tests/day9/sample").unwrap();
            for i in 0..heights.len() {
                for j in 0..heights[i].len() {
                    let want = matches!((i, j), (0, 1) | (0, 9) | (2, 2) | (4, 6));
                    let got = is_low_point(&heights, i, j);
                    assert_eq!(want, got);
                }
            }
        }
    }
}

fn main() {
    let input = "tests/day9/input";
    let heights = day9::load_heights("tests/day9/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day9::part1::solve(&heights));
    println!("{}", day9::part2::solve(&heights));
}
