#![allow(dead_code)]

use advent2021::ParseError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Copy)]
enum Relation {
    NoOverlap, // A: ranges do not overlap
    Identical, // B: old ranges coincide exactly
    OldInside, // C: old range fits entirely inside new
    NewTrails, // D: end of new range is past start of old
    OldTrails, // E: end of old range is past start of new
    NewInside, // F: new range fits entirely inside old
}

impl Relation {
    fn new(old: &Range<i32>, new: &Range<i32>) -> Relation {
        if old.start >= new.end || old.end <= new.start {
            Relation::NoOverlap // A: no overlap
        } else if old == new {
            Relation::Identical // B: pure overlap
        } else {
            match (old.contains(&new.start), old.contains(&new.end)) {
                (false, false) => Relation::OldInside, // C
                (false, true) => Relation::NewTrails,  // D
                (true, false) => Relation::OldTrails,  // E
                (true, true) => Relation::NewInside,   // F
            }
        }
    }
}

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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Cuboid {
    xs: Range<i32>,
    ys: Range<i32>,
    zs: Range<i32>,
}

impl Cuboid {
    /// Inclusive
    fn above(&self, z: i32) -> Cuboid {
        let zs = z..self.zs.end;
        Cuboid { zs, ..self.clone() }
    }

    /// Exclusive
    fn below(&self, z: i32) -> Cuboid {
        let zs = self.zs.start..z;
        Cuboid { zs, ..self.clone() }
    }

    /// Returns boxes representing portions of self not overlapped by that.
    fn receive(self, that: &Cuboid) -> impl Iterator<Item = Cuboid> {
        #[rustfmt::skip]
        use Relation::{
            NoOverlap as A, // The relationship of two cuboids is effectively a
            Identical as B, // three-digit, base 6 number, because cuboids
            OldInside as C, // are three-dimensional objects that may have any
            NewTrails as D, // of six Relations in any single dimension.  They
            OldTrails as E, // intersect in 5**3 of those 6**3 relationships,
            NewInside as F, // all covered by the following pattern match.
        };
        let Cuboid { xs, ys, zs } = that;
        match (
            Relation::new(&self.xs, xs),
            Relation::new(&self.ys, ys),
            Relation::new(&self.zs, zs),
        ) {
            (A, _, _) | (_, A, _) | (_, _, A) => vec![self], // no overlap
            (B | C, B | C, B | C) => Vec::new(),             // obliterated
            (B, B, D) => vec![self.above(zs.end)],
            (B, B, E) => vec![self.below(zs.start)],
            (B, B, F) => vec![self.below(zs.start), self.above(zs.end)],
            _ => todo!(),
        }
        .into_iter()
    }

    fn volume(&self) -> usize {
        let dx = (self.xs.end - self.xs.start) as usize;
        let dy = (self.ys.end - self.ys.start) as usize;
        let dz = (self.zs.end - self.zs.start) as usize;
        dx * dy * dz
    }
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
    block: Cuboid,
}

impl Step {
    fn constrain(&self, max: i32) -> Step {
        Step {
            state: self.state.clone(),
            block: Cuboid {
                xs: constrain_range(&self.block.xs, max),
                ys: constrain_range(&self.block.ys, max),
                zs: constrain_range(&self.block.zs, max),
            },
        }
    }

    fn cubes(&self) -> impl Iterator<Item = (i32, i32, i32)> + '_ {
        let xs = self.block.xs.clone();
        xs.flat_map(move |x| {
            let ys = self.block.ys.clone();
            ys.flat_map(move |y| {
                let zs = self.block.zs.clone();
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
            block: Cuboid {
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

pub mod part2 {
    use super::*;

    pub fn solve(puzzle: &Puzzle) -> usize {
        let mut on: Vec<Cuboid> = Vec::new();
        for step in puzzle.steps.iter() {
            on = on
                .into_iter()
                .flat_map(|block| block.receive(&step.block))
                .collect();
            // TODO: Replace all existing cuboids overlapped by this one with
            // sets of non-overlapped subcuboids.
            if let State::On = step.state {
                on.push(step.block.clone());
            }
        }
        on.into_iter().map(|cuboid| cuboid.volume()).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day22/sample2").unwrap();
            assert_eq!(2758514936282235, solve(&puzzle));
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
