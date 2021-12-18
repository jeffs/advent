use advent2021::ParseError;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Read as _;
use std::ops::Range;
use std::path::Path;

fn parse_range(s: &str) -> Result<Range<i32>, ParseError> {
    let (start, last) = s
        .split_once("..")
        .ok_or_else(|| ParseError::new(format!("bad range: {}", s)))?;
    let last: i32 = last.parse()?;
    let range = start.parse()?..(last + 1);
    if range.is_empty() {
        return Err(ParseError::new(format!("range is empty: {}", s)));
    }
    Ok(range)
}

pub struct Target {
    x: Range<i32>,
    y: Range<i32>,
}

impl Target {
    fn contains(&self, x: i32, y: i32) -> bool {
        self.x.contains(&x) && self.y.contains(&y)
    }

    pub fn from_file<P>(input: P) -> Result<Target, ParseError>
    where
        P: AsRef<Path>,
    {
        const PREFIX: &str = "target area: ";
        let input = input.as_ref();
        let mut text = String::new();
        File::open(input)?.read_to_string(&mut text)?;
        let text = text.trim_end();
        if !text.starts_with(PREFIX) {
            return Err(ParseError::in_file(input, "expected target area"));
        }
        let (x, y) = text[PREFIX.len()..]
            .split_once(", ")
            .ok_or_else(|| ParseError::in_file(input, "expected x and y ranges"))?;
        if !x.starts_with("x=") {
            return Err(ParseError::in_file(input, "expected x range"));
        }
        if !y.starts_with("y=") {
            return Err(ParseError::in_file(input, "expected y range"));
        }
        let (x, y) = (parse_range(&x[2..])?, parse_range(&y[2..])?);
        Ok(Target { x, y })
    }

    fn path(&self, mut vx: i32, mut vy: i32) -> Option<Vec<(i32, i32)>> {
        let mut path = Vec::new();
        let (mut x, mut y) = (0, 0);
        while x <= self.right() && y >= self.bottom() {
            path.push((x, y));
            if self.contains(x, y) {
                return Some(path);
            }
            x += vx;
            y += vy;
            if vx != 0 {
                vx -= vx.signum();
            }
            vy -= 1;
        }
        None
    }

    pub fn paths(&self) -> impl Iterator<Item = Vec<(i32, i32)>> + '_ {
        let (r, b) = (self.right(), self.bottom());
        (b..-b)
            .rev()
            .flat_map(move |vy| (-r..r).map(move |vx| (vx, vy)))
            .filter_map(|(vx, vy)| self.path(vx, vy))
    }

    pub fn print(&self, path: &[(i32, i32)]) {
        println!("\n{}\n", self.render(path));
    }

    fn render(&self, path: &[(i32, i32)]) -> String {
        let pt = path.iter().map(|p| p.1).max().expect("render empty path");
        let pb = path.iter().map(|p| p.1).min().expect("render empty path");
        let t = 0.max(pt).max(self.top());
        let r = 0.max(self.right());
        let b = 0.min(pb).min(self.bottom());
        let l = 0.min(self.left());
        let lines: Vec<_> = (b..=t)
            .rev()
            .map(|y| -> String {
                (l..r)
                    .map(|x| {
                        if (x, y) == (0, 0) {
                            'S'
                        } else if path.contains(&(x, y)) {
                            '#'
                        } else if self.contains(x, y) {
                            'T'
                        } else {
                            '.'
                        }
                    })
                    .collect()
            })
            .collect();
        lines.join("\n")
    }

    #[rustfmt::skip]    fn top(&self)    -> i32 { self.y.end }
    #[rustfmt::skip]    fn right(&self)  -> i32 { self.x.end }
    #[rustfmt::skip]    fn bottom(&self) -> i32 { self.y.start }
    #[rustfmt::skip]    fn left(&self)   -> i32 { self.x.start }
}

impl Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "target area: x={}..={}, y={}..={}",
            self.x.start,
            self.x.end - 1,
            self.y.start,
            self.y.end - 1
        )
    }
}

fn path_height(path: &[(i32, i32)]) -> i32 {
    *path.iter().map(|(_, y)| y).max().expect("empty path")
}

pub mod part1 {
    use super::{path_height, Target};

    pub fn solve(target: &Target) -> i32 {
        let path = target
            .paths()
            .max_by_key(|path| path_height(path))
            .expect("no solution");
        path_height(&path)
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Target};

        #[test]
        fn test_solve() {
            let target = Target::from_file("tests/day17/sample").unwrap();
            assert_eq!(45, solve(&target));
        }
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(target: &Target) -> usize {
        target.paths().count()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Target};

        #[test]
        fn test_solve() {
            let target = Target::from_file("tests/day17/sample").unwrap();
            assert_eq!(112, solve(&target));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{path_height, Target};

    #[test]
    fn test_path72() {
        let target = Target::from_file("tests/day17/sample").unwrap();
        let path = target.path(7, 2).expect("can't find sample path");
        assert_eq!(8, path.len());
        assert_eq!(3, path_height(&path));
    }

    #[test]
    fn test_path69() {
        let target = Target::from_file("tests/day17/sample").unwrap();
        let path = target.path(6, 9).expect("can't find sample path");
        assert_eq!(45, path_height(&path));
    }

    #[test]
    fn test_path_velocities() {
        #[rustfmt::skip]
        let want = [
            (23, -10), (25, -9), (27, -5), (29, -6), (22, -6), (21, -7), (9, 0), (27, -7), (24, -5),
            (25, -7), (26, -6), (25, -5), (6, 8), (11, -2), (20, -5), (29, -10), (6, 3), (28, -7),
            (8, 0), (30, -6), (29, -8), (20, -10), (6, 7), (6, 4), (6, 1), (14, -4), (21, -6),
            (26, -10), (7, -1), (7, 7), (8, -1), (21, -9), (6, 2), (20, -7), (30, -10), (14, -3),
            (20, -8), (13, -2), (7, 3), (28, -8), (29, -9), (15, -3), (22, -5), (26, -8), (25, -8),
            (25, -6), (15, -4), (9, -2), (15, -2), (12, -2), (28, -9), (12, -3), (24, -6), (23, -7),
            (25, -10), (7, 8), (11, -3), (26, -7), (7, 1), (23, -9), (6, 0), (22, -10), (27, -6),
            (8, 1), (22, -8), (13, -4), (7, 6), (28, -6), (11, -4), (12, -4), (26, -9), (7, 4),
            (24, -10), (23, -8), (30, -8), (7, 0), (9, -1), (10, -1), (26, -5), (22, -9), (6, 5),
            (7, 5), (23, -6), (28, -10), (10, -2), (11, -1), (20, -9), (14, -2), (29, -7), (13, -3),
            (23, -5), (24, -8), (27, -9), (30, -7), (28, -5), (21, -10), (7, 9), (6, 6), (21, -5),
            (27, -10), (7, 2), (30, -9), (21, -8), (22, -7), (24, -9), (20, -6), (6, 9), (29, -5),
            (8, -2), (27, -8), (30, -5), (24, -7),
        ];
        let target = Target::from_file("tests/day17/sample").unwrap();
        for (vx, vy) in want {
            assert!(target.path(vx, vy).is_some())
        }
    }
}

fn main() {
    let input = "tests/day17/input";
    let target = Target::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&target));
    println!("{}", part2::solve(&target));
}
