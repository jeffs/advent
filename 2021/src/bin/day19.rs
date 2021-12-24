#![allow(dead_code, unused_mut, unused_variables)]

use advent2021::ParseError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::ops::{Add, Sub};
use std::path::Path;

/// The minimum number of two beacons having identical (relative offsets to
/// each other) in two separate cubes to establish that the cubes overlap.
const MIN_OVERLAP: usize = 12;

type Offset = (i32, i32, i32); // dx, dy, dz

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Beacon(i32, i32, i32); // x, y, z

impl Beacon {
    /// Rotates this Beacon a quarter turn counterclockwise around the Y axis.
    fn orbit_y_left(&mut self) {
        let Beacon(x, y, z) = *self;
        *self = Beacon(-z, y, x);
    }

    /// Rotates this Beacon a quarter turn clockwise around the Y axis.
    fn orbit_y_right(&mut self) {
        let Beacon(x, y, z) = *self;
        *self = Beacon(z, y, -x);
    }

    /// Rotates this Beacon a quarter turn clockwise around the Y axis.
    fn orbit_z_right(&mut self) {
        let Beacon(x, y, z) = *self;
        *self = Beacon(y, -x, z);
    }
}

impl Add<Offset> for Beacon {
    type Output = Beacon;

    fn add(self, other: Offset) -> Self::Output {
        Beacon(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for &Beacon {
    type Output = Offset;

    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

fn next_coord<'a, I>(coords: &mut I) -> Result<i32, ParseError>
where
    I: Iterator<Item = &'a str>,
{
    Ok(coords
        .next()
        .ok_or_else(|| ParseError::new("expected coordinate"))?
        .parse()?)
}

fn parse_beacon(line: &str) -> Result<Beacon, ParseError> {
    let coords = &mut line.split(',');
    let beacon = Beacon(
        next_coord(coords)?,
        next_coord(coords)?,
        next_coord(coords)?,
    );
    if let Some(extra) = coords.next() {
        let what = format!("expected end of line, not {}", extra);
        return Err(ParseError::new(what));
    }
    Ok(beacon)
}

fn parse_beacons<E, I>(lines: &mut I) -> Result<Vec<Beacon>, ParseError>
where
    I: Iterator<Item = Result<String, E>>,
    ParseError: From<E>,
{
    let mut beacons = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        beacons.push(parse_beacon(&line)?);
    }
    Ok(beacons)
}

/// The detection cube of a single scanner.
#[derive(Clone, Eq, Hash, PartialEq)]
struct Cube {
    beacons: Vec<Beacon>,
}

impl Cube {
    fn has_beacon(&self, beacon: &Beacon) -> bool {
        self.beacons.iter().position(|b| b == beacon).is_some()
    }

    fn from_lines<E, I>(lines: &mut I) -> Result<Option<Cube>, ParseError>
    where
        I: Iterator<Item = Result<String, E>>,
        ParseError: From<E>,
    {
        if let Some(line) = lines.next() {
            let line = line?;
            if !line.starts_with("--- scanner ") {
                return Err(ParseError::new("expected scanner"));
            }
            let beacons = parse_beacons(lines)?;
            Ok(Some(Cube { beacons }))
        } else {
            Ok(None)
        }
    }

    fn marry_beacons(&self, cube: &Cube, offset: Offset) -> bool {
        self.beacons
            .iter()
            .map(|&b| b + offset)
            .filter(|b| cube.has_beacon(b))
            .count()
            >= MIN_OVERLAP
    }

    /// Attempts to identify beacons shared with the specified other cube.
    /// Returns a copy of this cube's beacons on success, translated to be
    /// relative to the other cube's origin, and None on failure.
    fn marry_rotation(&self, cube: &Cube) -> Option<Vec<Beacon>> {
        self.beacons
            .iter()
            .flat_map(|ours| cube.beacons.iter().map(|theirs| theirs - ours))
            .find(|&offset| self.marry_beacons(cube, offset))
            .map(|offset| self.beacons.iter().cloned().map(|b| b + offset).collect())
    }

    /// Attempts to orient this cube to align it with the specified other cube.
    /// The cubes are aligned if, after some rectilinear translation, the two
    /// contain at least 12 beacons at the same positions.  Returns a copy of
    /// this cubes's beacons on success, transformed to the other cube's frame
    /// of reference, and None on failure.  This method may rotate this cube
    /// (even on failure), but does not translate it.
    #[rustfmt::skip]
    fn marry(&mut self, cube: &Cube) -> Option<Vec<Beacon>> {
        // Head up, initially facing north
        self.marry_rotation(cube)                                         //  0 face north
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) //  1 face west
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) //  2 face south
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) //  3 face east

            // Head north
            .or_else(|| { self.fall_left();  self.marry_rotation(cube) }) //  4 face east
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) //  5 face down
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) //  6 face west
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) //  7 face up

            // Head east
            .or_else(|| { self.fall_left();  self.marry_rotation(cube) }) //  8 face up
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) //  9 face south
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 10 face down
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 11 face north

            // Head down
            .or_else(|| { self.fall_right(); self.marry_rotation(cube) }) // 12 face north
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 13 face east
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 14 face south
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 15 face west

            // Head south
            .or_else(|| { self.fall_right(); self.marry_rotation(cube) }) // 16 face west
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 17 face down
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 18 face east
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 19 face up

            // Head west
            .or_else(|| { self.fall_left();  self.marry_rotation(cube) }) // 16 face up
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 17 face north
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 18 face down
            .or_else(|| { self.turn_left();  self.marry_rotation(cube) }) // 19 face south
    }

    fn fall_left(&mut self) {
        self.beacons.iter_mut().for_each(|b| b.orbit_y_right());
    }

    fn fall_right(&mut self) {
        self.beacons.iter_mut().for_each(|b| b.orbit_y_left());
    }

    fn turn_left(&mut self) {
        self.beacons.iter_mut().for_each(|b| b.orbit_z_right());
    }
}

pub struct Puzzle {
    cubes: Vec<Cube>,
}

impl Puzzle {
    fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let mut cubes = Vec::new();
        let lines = &mut BufReader::new(File::open(input)?).lines();
        while let Some(cube) = Cube::from_lines(lines)? {
            cubes.push(cube);
        }
        Ok(Puzzle { cubes })
    }
}

pub mod part1 {
    use super::*;

    #[derive(Clone, Copy, Eq, PartialEq)]
    enum Task {
        Orient, // Cubes begin life disoriented.
        Visit,  // Once oriented, they can visit others to help orient them.
        Retire, // After visiting all disoriented others, a cube retires.
    }

    use Task::*;

    pub fn solve(Puzzle { mut cubes }: Puzzle) -> usize {
        //  Mark the first cube (only) oriented.
        //  While there are unvisited cubes:
        //      Select an unvisited cube A.
        //      For each disoriented cube B:
        //          Attempt to orient B relative to A.
        //          If successful, mark B unvisited.
        //      Mark A visited.
        //  There should be no more disoriented cubes.
        //  Return the number of unique beacons across all cubes.
        let n = cubes.len();
        let mut beacons = HashSet::new();
        let mut tasks = vec![Orient; n];
        tasks[0] = Visit;
        while let Some(i) = tasks.iter().position(|&t| t == Visit) {
            // Clone the cube we're visiting, to work around Rust's borrowing
            // restrictions.  We'll be accessing other cubes mutably from the
            // same container at the same time.  Even though we can be sure
            // they're different cubes (since they have different Task values),
            // the Rust compiler sees only two conflicting attempts to borrow
            // the same container.  Each cube is visited at most once.
            let a = cubes[i].clone();
            for j in 0..n {
                if tasks[j] == Orient {
                    if let Some(transformed) = cubes[j].marry(&a) {
                        beacons.extend(transformed);
                        tasks[j] = Visit;
                    }
                }
            }
            tasks[i] = Retire;
        }
        if tasks.iter().any(|&t| t == Orient) {
            panic!("not all cubes overlap");
        }
        beacons.extend(cubes[0].beacons.iter());
        beacons.len()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day19/sample").unwrap();
            assert_eq!(79, solve(puzzle));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;

    #[test]
    fn test_marry_rotation() {
        let puzzle = Puzzle::from_file("tests/day19/sample").unwrap();
        let (cube0, cube1) = (&puzzle.cubes[0], &puzzle.cubes[1]);
        assert!(cube0.marry_rotation(cube0).is_some());
        assert!(cube0.marry_rotation(cube1).is_none());
    }

    #[test]
    fn test_marry() {
        let puzzle = Puzzle::from_file("tests/day19/sample").unwrap();
        let (cube0, cube1) = (&mut puzzle.cubes[0].clone(), &puzzle.cubes[1]);
        let orig0 = &puzzle.cubes[0];
        assert!(cube0.marry(orig0).is_some()); // head up, face north
        assert!(cube0.marry(cube1).is_some()); // head south, face down
    }
}

fn main() {
    let input = "tests/day19/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(puzzle));
}
