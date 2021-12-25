use crate::beacon::Beacon;
use crate::cube::Cube;
use advent2021::ParseError;
use std::fs::File;
use std::io;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

fn parse_beacons<I>(lines: &mut I) -> Result<Vec<Beacon>, ParseError>
where
    I: Iterator<Item = Result<String, io::Error>>,
{
    let mut beacons = Vec::new();
    for line in lines {
        let line = line?;
        if line.is_empty() {
            break;
        }
        beacons.push(line.parse()?);
    }
    Ok(beacons)
}

impl Cube {
    pub fn from_lines<I>(lines: &mut I) -> Result<Option<Cube>, ParseError>
    where
        I: Iterator<Item = Result<String, io::Error>>,
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
}

pub fn cubes_from_file<P>(input: P) -> Result<Vec<Cube>, ParseError>
where
    P: AsRef<Path>,
{
    let mut cubes = Vec::new();
    let lines = &mut BufReader::new(File::open(input)?).lines();
    while let Some(cube) = Cube::from_lines(lines)? {
        cubes.push(cube);
    }
    Ok(cubes)
}
