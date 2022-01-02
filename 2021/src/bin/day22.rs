use advent2021::ParseError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum State {
    Off,
    On,
}

impl FromStr for State {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "off" => Ok(State::Off),
            "on" => Ok(State::On),
            _ => Err(ParseError::new("state must be off on or on")),
        }
    }
}

#[derive(Debug)]
struct Cuboid {
    xs: Range<i32>,
    ys: Range<i32>,
    zs: Range<i32>,
}

fn constrain_range(range: &Range<i32>, max: i32) -> Range<i32> {
    range.start.max(-max)..range.end.min(max + 1)
}

fn parse_range(s: &str) -> Result<Range<i32>, ParseError> {
    if s.len() < "x=1..1".len() {
        return Err(ParseError::new("bad range: too short"));
    }
    let (min, max) = s[2..]
        .split_once("..")
        .ok_or_else(|| ParseError::new("bad range: expected '..'"))?;
    let max: i32 = max.parse()?;
    Ok(min.parse()?..(max + 1))
}

#[derive(Debug)]
struct Step {
    state: State,
    cubes: Cuboid,
}

impl Step {
    fn constrain(&self, max: i32) -> Step {
        Step {
            state: self.state.clone(),
            cubes: Cuboid {
                xs: constrain_range(&self.cubes.xs, max),
                ys: constrain_range(&self.cubes.ys, max),
                zs: constrain_range(&self.cubes.zs, max),
            },
        }
    }

    fn cubes(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        let xs = self.cubes.xs.clone();
        xs.flat_map(move |x| {
            let ys = self.cubes.ys.clone();
            ys.flat_map(move |y| {
                let zs = self.cubes.zs.clone();
                zs.map(move |z| (x, y, z))
            })
        })
    }
}

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < "on x=1..1,y=1..1,z=1..1".len() {
            return Err(ParseError::new("bad step: too short"));
        }
        let ranges: Vec<_> = s[3..].split(',').collect();
        if ranges.len() != 3 {
            return Err(ParseError::new("bad step: expected three ranges"));
        }
        let step = Step {
            state: s[..3].parse()?,
            cubes: Cuboid {
                xs: parse_range(ranges[0].trim())?,
                ys: parse_range(ranges[1].trim())?,
                zs: parse_range(ranges[2].trim())?,
            },
        };
        Ok(step)
    }
}

#[derive(Debug)]
pub struct Puzzle {
    steps: Vec<Step>,
}

impl Puzzle {
    fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut steps = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            steps.push(line?.parse()?);
        }
        Ok(Puzzle { steps })
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(puzzle: &Puzzle) -> usize {
        let mut reactor = HashSet::new(); // which cubes are on
        for step in &puzzle.steps {
            let step = step.constrain(50);
            for cube in step.cubes() {
                match step.state {
                    State::Off => reactor.remove(&cube),
                    State::On => reactor.insert(cube),
                };
            }
        }
        reactor.len()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day22/sample").unwrap();
            assert_eq!(590784, solve(&puzzle));
        }
    }
}

fn main() {
    let input = "tests/day22/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&puzzle));
}
