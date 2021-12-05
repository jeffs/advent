#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

use advent2021::{EmptyFile, NoSolution, ParseError};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

mod day5 {
    use super::*;

    #[derive(Debug)]
    struct Point {
        x: usize,
        y: usize,
    }

    impl FromStr for Point {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some((x, y)) = s.split_once(',') {
                Ok(Point {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            } else {
                Err(ParseError::new(format!("bad point: {}", s)))
            }
        }
    }

    #[derive(Debug)]
    pub struct Segment {
        p1: Point,
        p2: Point,
    }

    impl Segment {
        fn is_horizontal(&self) -> bool {
            self.p1.y == self.p2.y
        }

        fn is_vertical(&self) -> bool {
            self.p1.x == self.p2.x
        }
    }

    impl FromStr for Segment {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some((p1, p2)) = s.split_once(" -> ") {
                Ok(Segment {
                    p1: p1.parse()?,
                    p2: p2.parse()?,
                })
            } else {
                Err(ParseError::new(format!("bad segment: {}", s)))
            }
        }
    }

    #[derive(Default)]
    struct Floor {
        counts: Vec<u32>, // number of overlapping vents at each point
        width: usize,
    }

    fn normalize(segment: &Segment) -> Segment {
        let Segment { p1, p2 } = segment;
        let (x1, x2) = if p1.x <= p2.x {
            (p1.x, p2.x)
        } else {
            (p2.x, p1.x)
        };
        let (y1, y2) = if p1.y <= p2.y {
            (p1.y, p2.y)
        } else {
            (p2.y, p1.y)
        };
        Segment {
            p1: Point { x: x1, y: y1 },
            p2: Point { x: x2, y: y2 },
        }
    }

    impl Floor {
        fn mark(&mut self, segment: &Segment) {
            let Segment {
                p1: Point { x: x1, y: y1 },
                p2: Point { x: x2, y: y2 },
            } = normalize(segment);
            if x1 == x2 {
                for y in y1..=y2 {}
            }
            //for x in x1..x2
            // let Point { x: x1, y: y1 } = p1;
            // let Point { x: x2, y: y2 } = p2;
            // let x_step = if x1 > x2 { -1 } else { 1 };
            // let y_step = if y1 > y2 { -1 } else { 1 };
        }

        fn resize(&mut self, width: usize, height: usize) {
            let mut counts = vec![0; width * height];
            for i in 0..self.height() {
                for j in 0..self.width {
                    counts[i * w + j] = self.counts[i * self.width + j];
                }
            }
            self.counts = std::mem::take(&mut counts);
        }

        fn expand(&mut self, p: &Point) {
            let (w, h) = (p.x + 1, p.y + 1);
            match (w > self.width, h > self.height()) {
                (false, false) => (),
                (false, true) => self.counts.resize(self.width * h),
                (true, false) => self.resize(w, self.height()),
                (true, true) => self.resize(w, h),
            }
        }

        fn height(&self) {
            self.counts.len() / self.width
        }

        fn incr(&mut self, p: &Point) {
            self.ensure_size()
        }
    }

    pub fn load_segments<P: AsRef<Path>>(input: P) -> Result<Vec<Segment>, Box<dyn Error>> {
        let mut segments = Vec::new();
        for line in BufReader::new(File::open(&input)?).lines() {
            segments.push(line?.parse()?);
        }
        Ok(segments)
    }

    pub mod part1 {
        use super::*;

        pub fn solve(segments: &[Segment]) -> u32 {
            let mut floor = Floor::default();
            for segment in segments {
                if segment.is_horizontal() || segment.is_vertical() {
                    floor.mark(&segment);
                }
            }
            todo!()
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                let segments = load_segments("tests/day5/sample").unwrap();
                assert_eq!(5, solve(&segments));
            }
        }
    }
}

fn main() {
    let input = "tests/day5/input";
    let segments = day5::load_segments("tests/day5/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day5::part1::solve(&segments));
}
