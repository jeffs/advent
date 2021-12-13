use advent2021::ParseError;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;
use std::str::FromStr;

mod day13 {
    use super::*;

    type Point = (usize, usize); // (x, y)

    #[derive(Clone, Copy)]
    enum Fold {
        X(usize), // fold left
        Y(usize), // fold up
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

    #[derive(Clone)]
    pub struct Page {
        points: HashSet<Point>,
    }

    impl Page {
        fn fold_left(&mut self, x: usize) {
            let olds: Vec<_> = self.points.iter().cloned().filter(|p| p.0 > x).collect();
            for p in olds {
                self.points.insert((x - (p.0 - x), p.1));
                self.points.remove(&p);
            }
        }

        fn fold_up(&mut self, y: usize) {
            let olds: Vec<_> = self.points.iter().cloned().filter(|p| p.1 > y).collect();
            for p in olds {
                self.points.insert((p.0, y - (p.1 - y)));
                self.points.remove(&p);
            }
        }

        fn fold(&mut self, fold: Fold) {
            match fold {
                Fold::X(index) => self.fold_left(index),
                Fold::Y(index) => self.fold_up(index),
            }
        }

        fn height(&self) -> usize {
            self.points
                .iter()
                .map(|p| p.1 + 1)
                .max()
                .unwrap_or_default()
        }

        fn width(&self) -> usize {
            self.points
                .iter()
                .map(|p| p.0 + 1)
                .max()
                .unwrap_or_default()
        }
    }

    impl Display for Page {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let (w, h) = (self.width(), self.height());
            for y in 0..h {
                for x in 0..w {
                    let c = if self.points.contains(&(x, y)) {
                        '#'
                    } else {
                        ' '
                    };
                    write!(f, "{}", c)?;
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }

    #[derive(Clone)]
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

        pub fn solve(mut puzzle: Puzzle) -> usize {
            let first = *puzzle.folds.first().expect("missing fold");
            puzzle.page.fold(first);
            puzzle.page.points.len()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_puzzle;
            use super::solve;

            #[test]
            fn test_solve() {
                let puzzle = load_puzzle("tests/day13/sample").unwrap();
                assert_eq!(17, solve(puzzle));
            }
        }
    }

    pub mod part2 {
        use super::*;

        pub fn solve(mut puzzle: Puzzle) -> Page {
            puzzle.folds.iter().for_each(|&f| puzzle.page.fold(f));
            puzzle.page
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_puzzle;
            use super::solve;

            #[test]
            fn test_solve() {
                let want = "#####\n\
                            #   #\n\
                            #   #\n\
                            #   #\n\
                            #####\n";
                let puzzle = load_puzzle("tests/day13/sample").unwrap();
                assert_eq!(want, format!("{}", solve(puzzle)));
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
    println!("{}", day13::part1::solve(puzzle.clone()));
    println!("{}", day13::part2::solve(puzzle));
}
