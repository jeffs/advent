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
            let rows = (1.max(start.0) - 1)..cave.len().min(start.1 + 2);
            rows.flat_map(|i| {
                let columns = (1.max(start.1) - 1)..cave[i].len().min(start.1 + 2);
                columns.map(move |j| (i, j))
            })
            //.filter(|(i, j)| (0..cave.len()).contains(i) && (0..cave[i].len()).contains(j))
            .collect()
        }

        fn recur(cave: &Cave, start: Point, seen: &mut HashSet<Point>) -> Option<usize> {
            assert!(!seen.contains(&start));
            // let indent = " ".repeat(seen.len());
            // println!("{} ({}, {})", indent, start.0, start.1);
            let end = (cave.len() - 1, cave[cave.len() - 1].len() - 1);
            if start == end {
                println!("{:?}", seen);
                Some(0)
            } else {
                seen.insert(start);
                let mut kids = neighbors(cave, start);
                kids.retain(|p| !seen.contains(p));
                let result = kids
                    .into_iter()
                    .filter_map(|p| recur(cave, p, seen).map(|d| d + cave[p.0][p.1] as usize))
                    .min();
                seen.remove(&start);
                result
            }
        }

        pub fn solve(cave: &Cave) -> usize {
            let mut seen = HashSet::new();
            recur(&cave, (0, 0), &mut seen).expect("no solution")
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_cave;
            use super::solve;

            #[test]
            fn test_solve_tiny() {
                //[(2, 4), (3, 14)].into_iter().for_each(|(size, want)| {
                [(3, 14)].into_iter().for_each(|(size, want)| {
                    let file = format!("tests/day15/tiny{}", size);
                    let cave = load_cave(file).unwrap();
                    assert_eq!(want, solve(&cave));
                });
            }

            // #[test]
            // fn test_solve() {
            //     let cave = load_cave("tests/day15/sample").unwrap();
            //     assert_eq!(40, solve(&cave));
            // }
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
