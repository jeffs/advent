use advent2021::ParseError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day15 {
    use super::*;

    type Cave = Vec<Vec<u8>>;

    pub fn load_cave<P>(input: P) -> Result<Cave, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut cave = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            cave.push(line?.bytes().map(|b| b - b'0').collect());
        }
        Ok(cave)
    }

    pub mod part1 {
        use super::*;

        type Point = (usize, usize); // i, j

        fn neighbors(cave: &Cave, start: Point) -> Vec<Point> {
            let rows = (1.max(start.0) - 1)..cave.len().min(start.0 + 2);
            rows.flat_map(|i| {
                let columns = (1.max(start.1) - 1)..cave[i].len().min(start.1 + 2);
                columns.map(move |j| (i, j))
            })
            .collect()
        }

        type CavePath = Vec<Point>;

        fn path_cost(cave: &Cave, path: &CavePath) -> usize {
            path.iter().skip(1).map(|&(i, j)| cave[i][j] as usize).sum()
        }

        fn recur(cave: &Cave, path: &mut CavePath, here: Point, mut best: usize) -> Vec<CavePath> {
            let mut paths = Vec::new();
            path.push(here);
            if here == (cave.len() - 1, cave[cave.len() - 1].len() - 1) {
                // We've reached our goal, the bottom right corner.
                paths.push(path.clone());
            } else {
                let cost = path_cost(cave, path);
                let mut kids = neighbors(cave, here);
                kids.retain(|p| !path.contains(p)); // avoid cycles
                kids.retain(|&(i, j)| cost + (cave[i][j] as usize) < best); // prune expensive paths
                for kid in kids {
                    let news = recur(cave, path, kid, best);
                    best = news
                        .iter()
                        .map(|new| path_cost(cave, new))
                        .min()
                        .unwrap_or(best);
                    paths.extend(news.into_iter());
                }
            }
            path.pop();
            paths
        }

        pub fn solve(cave: &Cave) -> usize {
            let paths = recur(&cave, &mut Vec::new(), (0, 0), usize::MAX);
            let distinct: HashSet<_> = paths.iter().collect();
            assert_eq!(distinct.len(), paths.len());
            let path = paths
                .into_iter()
                .min_by_key(|path| path_cost(cave, &path))
                .expect("no solution");
            println!("{:?}", path);
            path_cost(cave, &path)
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_cave;
            use super::solve;

            #[test]
            fn test_solve_tiny() {
                [(2, 4), (3, 14)].into_iter().for_each(|(size, want)| {
                    let file = format!("tests/day15/tiny{}", size);
                    let cave = load_cave(file).unwrap();
                    assert_eq!(want, solve(&cave));
                });
            }

            #[test]
            fn test_solve() {
                let cave = load_cave("tests/day15/sample").unwrap();
                assert_eq!(40, solve(&cave));
            }
        }
    }
}

fn main() {
    let input = "tests/day15/input";
    let cave = day15::load_cave(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day15::part1::solve(&cave));
}
