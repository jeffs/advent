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

pub mod part1 {
    use super::*;

    fn path_height(path: &[(i32, i32)]) -> i32 {
        *path.iter().map(|(_, y)| y).max().expect("empty path")
    }

    pub fn solve(target: &Target) -> i32 {
        let (r, b) = (target.right(), target.bottom());
        let path = (0..-b)
            .rev()
            .flat_map(move |vy| (0..r).map(move |vx| (vx, vy)))
            .filter_map(|(vx, vy)| target.path(vx, vy))
            .max_by_key(|path| path_height(path))
            .expect("no solution");
        path_height(&path)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

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
        fn test_solve() {
            let target = Target::from_file("tests/day17/sample").unwrap();
            assert_eq!(45, solve(&target));
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
}
