#![allow(dead_code, unused_mut, unused_variables)]

use advent2021::ParseError;
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
    fn marry_rotation(&self, cube: &Cube) -> bool {
        self.beacons
            .iter()
            .flat_map(|ours| cube.beacons.iter().map(|theirs| theirs - ours))
            .any(|offset| self.marry_beacons(cube, offset))
    }

    /// Attempts to orient this cube to align it with the specified other cube.
    /// The cubes are aligned if, after some rectilinear translation, the two
    /// contain at least 12 beacons at the same positions.  Returns true on
    /// success, and false on failure.  Even on failure, does not necessarily
    /// leave this cube in its original orientation.
    fn marry(&mut self, cube: &Cube) -> bool {
        // Head up, initially facing north
        self.marry_rotation(cube)                               //  0 face north
            || (self.turn_left(),  self.marry_rotation(cube)).1 //  1 face west
            || (self.turn_left(),  self.marry_rotation(cube)).1 //  2 face south
            || (self.turn_left(),  self.marry_rotation(cube)).1 //  3 face east

            // Head north
            || (self.fall_left(),  self.marry_rotation(cube)).1 //  4 face east
            || (self.turn_left(),  self.marry_rotation(cube)).1 //  5 face down
            || (self.turn_left(),  self.marry_rotation(cube)).1 //  6 face west
            || (self.turn_left(),  self.marry_rotation(cube)).1 //  7 face up

            // Head east
            || (self.fall_left(),  self.marry_rotation(cube)).1 //  8 face up
            || (self.turn_left(),  self.marry_rotation(cube)).1 //  9 face south
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 10 face down
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 11 face north

            // Head down
            || (self.fall_right(), self.marry_rotation(cube)).1 // 12 face north
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 13 face east
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 14 face south
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 15 face west

            // Head south
            || (self.fall_right(), self.marry_rotation(cube)).1 // 16 face west
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 17 face down
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 18 face east
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 19 face up

            // Head west
            || (self.fall_left(),  self.marry_rotation(cube)).1 // 16 face up
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 17 face north
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 18 face down
            || (self.turn_left(),  self.marry_rotation(cube)).1 // 19 face south
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
        let mut tasks = vec![Orient; n];
        tasks[0] = Visit;
        while let Some(i) = tasks.iter().position(|&t| t == Visit) {
            // Clone the cube we're visiting, to work around Rust's borrowing
            // restrictions.  We'll be accessing other cubes mutably from the
            // same container at the same time.  Even though we can be sure
            // they're different cubes (since they have different Task values),
            // the Rust compiler sees only two conflicting attempts to borrow
            // the same container.  Each cube will visited at most once.
            let a = cubes[i].clone();
            for j in 0..n {
                if tasks[j] == Orient && cubes[j].marry(&a) {
                    tasks[j] = Visit;
                }
            }
            tasks[i] = Retire;
        }
        if tasks.iter().any(|&t| t == Orient) {
            panic!("not all cubes overlap");
        }
        // TODO This won't work.  You need to know how many _distinct_ beacons
        // exist, account for translations to different scanner cubes.
        cubes.iter().map(|cube| cube.beacons.len()).sum()
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
        assert!(cube0.marry_rotation(cube0));
        assert!(!cube0.marry_rotation(cube1));
    }

    #[test]
    fn test_marry() {
        let puzzle = Puzzle::from_file("tests/day19/sample").unwrap();
        let (cube0, cube1) = (&mut puzzle.cubes[0].clone(), &puzzle.cubes[1]);
        assert!(cube0.marry(&puzzle.cubes[0])); // head up, face north
        assert!(cube0.marry(cube1)); // head south, face down
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
