use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::num::ParseIntError;
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

// ParseError

#[derive(Debug)]
struct ParseError {
    what: String,
}

impl ParseError {
    fn new(what: &str) -> ParseError {
        ParseError {
            what: what.to_owned(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl Error for ParseError {}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> Self {
        Self::new(&err.to_string())
    }
}

// Policy

fn parse_letter(s: &str) -> Result<char, ParseError> {
    match &s.chars().collect::<Vec<char>>()[..] {
        &[c] => Ok(c),
        _ => Err(ParseError::new(&format!(r#"bad letter: "{}""#, s))),
    }
}

fn parse_range(s: &str) -> Result<Range<u32>, ParseError> {
    match s.splitn(2, '-').collect::<Vec<&str>>()[..] {
        [min, max] => Ok(Range {
            start: min.parse()?,
            end: max.parse::<u32>()? + 1,
        }),
        _ => Err(ParseError::new(&format!("bad range: {}", s))),
    }
}

#[derive(Debug)]
struct Policy {
    range: Range<u32>,
    letter: char,
}

impl FromStr for Policy {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.splitn(2, ' ').collect::<Vec<&str>>()[..] {
            [range, letter] => Ok(Policy {
                range: parse_range(range)?,
                letter: parse_letter(letter)?,
            }),
            _ => Err(ParseError::new(&format!("bad policy: {}", s))),
        }
    }
}

// Entry

#[derive(Debug)]
struct Entry {
    policy: Policy,
    password: String,
}

impl FromStr for Entry {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.rsplitn(2, ": ").collect::<Vec<&str>>()[..] {
            [password, policy] => Ok(Entry {
                policy: policy.parse()?,
                password: password.to_owned(),
            }),
            _ => Err(ParseError::new(&format!("bad entry: {}", s))),
        }
    }
}

fn load_entries<P: AsRef<Path>>(input: P) -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut entries = Vec::new();
    for line in BufReader::new(File::open(input)?).lines() {
        entries.push(line?.parse()?);
    }
    Ok(entries)
}

// Part 1

fn solve_part1(_entries: &Vec<Entry>) -> u32 {
    todo!()
}

// Main

fn main() {
    let input = "tests/day2/input";
    let entries = load_entries(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{:#?}", entries);
    println!("{}", solve_part1(&entries));
}
