use advent2021::ParseError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;
use std::str::FromStr;

mod day13 {
    use super::*;

    enum Fold {
        X(usize),
        Y(usize),
    }

    impl FromStr for Fold {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let bytes = s.as_bytes();
            if bytes.len() < 3 || bytes[1] != b'=' {
                return Err(ParseError::new(format!("bad fold: {}", s)));
            }
            let (axis, index) = (bytes[0], s[2..].parse()?);
            Ok(match axis {
                b'x' => Fold::X(index),
                b'y' => Fold::Y(index),
                _ => {
                    let what = format!("bad fold; axis must be x or y: {}", s);
                    return Err(ParseError::new(what));
                }
            })
        }
    }

    pub struct Page {
        points: HashSet<(usize, usize)>,
    }

    pub struct Puzzle {
        page: Page,
        folds: Vec<Fold>,
    }

    fn load_page<E, I>(lines: &mut I) -> Result<Page, ParseError>
    where
        E: Into<ParseError>,
        I: Iterator<Item = Result<String, E>>,
    {
        let mut points = HashSet::new();
        while let Some(line) = lines.next() {
            let line = line.map_err(|e| e.into())?;
            if line.is_empty() {
                break;
            }
            let (x, y) = line
                .split_once(',')
                .ok_or_else(|| ParseError::new(format!("bad point: {}", line)))?;
            points.insert((x.parse()?, y.parse()?));
        }
        Ok(Page { points })
    }

    fn load_folds<E, I>(lines: &mut I) -> Result<Vec<Fold>, ParseError>
    where
        E: Into<ParseError>,
        I: Iterator<Item = Result<String, E>>,
    {
        const FOLD_PREFIX: &str = "fold along ";
        let mut folds = Vec::new();
        while let Some(line) = lines.next() {
            let line = line.map_err(|e| e.into())?;
            if !line.starts_with(FOLD_PREFIX) {
                let what = format!("expected fold; got: {}", line);
                return Err(ParseError::new(what));
            }
            folds.push(line[FOLD_PREFIX.len()..].parse()?);
        }
        Ok(folds)
    }

    pub fn load_puzzle<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut lines = BufReader::new(File::open(input)?).lines();
        Ok(Puzzle {
            page: load_page(&mut lines)?,
            folds: load_folds(&mut lines)?,
        })
    }

    pub mod part1 {
        use super::*;

        pub fn solve(puzzle: &Puzzle) -> usize {
            puzzle.page.points.len()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_puzzle;
            use super::solve;

            #[test]
            fn test_solve() {
                let lines = load_puzzle("tests/day13/sample").unwrap();
                assert_eq!(17, solve(&lines));
            }
        }
    }
}

fn main() {
    let input = "tests/day13/input";
    let puzzle = day13::load_puzzle(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day13::part1::solve(&puzzle));
}
