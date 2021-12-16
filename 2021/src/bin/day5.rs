use advent2021::ParseError;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

type Point = (isize, isize);

fn parse_point(s: &str) -> Result<Point, ParseError> {
    if let Some((x, y)) = s.split_once(',') {
        Ok((x.parse()?, y.parse()?))
    } else {
        Err(ParseError::new(format!("bad point: {}", s)))
    }
}

type Segment = (Point, Point);

fn parse_segment(s: &str) -> Result<Segment, ParseError> {
    if let Some((p1, p2)) = s.split_once(" -> ") {
        Ok((parse_point(p1)?, parse_point(p2)?))
    } else {
        Err(ParseError::new(format!("bad segment: {}", s)))
    }
}

fn load_segments<P: AsRef<Path>>(input: P) -> Result<Vec<Segment>, Box<dyn Error>> {
    let mut segments = Vec::new();
    for line in BufReader::new(File::open(&input)?).lines() {
        segments.push(parse_segment(&line?)?);
    }
    Ok(segments)
}

pub fn solve(segments: &[Segment]) -> usize {
    let mut floor: HashMap<(isize, isize), usize> = HashMap::new();
    for &((x1, y1), (x2, y2)) in segments {
        let (dx, dy) = ((x2 - x1).signum(), (y2 - y1).signum());
        let (w, h) = ((x2 - x1).abs() + 1, (y2 - y1).abs() + 1);
        for i in 0..w.max(h) {
            let point = (x1 + i * dx, y1 + i * dy);
            *floor.entry(point).or_default() += 1;
        }
    }
    floor.into_values().filter(|&n| n > 1).count()
}

fn _print_floor(floor: &HashMap<(isize, isize), usize>) {
    println!();
    for i in 0..10 {
        print!("  ");
        for j in 0..10 {
            let c = floor
                .get(&(i, j))
                .map_or('.', |&n| ('0' as usize + n) as u8 as char);
            print!("{}", c);
        }
        println!();
    }
    println!();
}

pub mod part1 {
    use super::*;

    pub fn solve(segments: &[Segment]) -> usize {
        let segments: Vec<_> = segments
            .iter()
            .cloned()
            .filter(|&((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
            .collect();
        super::solve(&segments)
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

pub mod part2 {
    use super::*;

    pub use super::solve;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            let segments = load_segments("tests/day5/sample").unwrap();
            assert_eq!(12, solve(&segments));
        }
    }
}

fn main() {
    let input = "tests/day5/input";
    let segments = load_segments("tests/day5/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&segments));
    println!("{}", part2::solve(&segments));
}
