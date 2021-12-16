use advent2021::ParseError;
use std::fs::File;
use std::io::{self, BufRead as _, BufReader, Lines};
use std::path::Path;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Clone)]
pub struct Command(Axis, i32);

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s
            .split_once(' ')
            .ok_or_else(|| ParseError::new(format!("bad command: {}", s)))?;
        let distance = distance.parse()?;
        let command = match direction {
            "forward" => Command(Axis::Horizontal, distance),
            "down" => Command(Axis::Vertical, distance),
            "up" => Command(Axis::Vertical, -distance),
            _ => return Err(ParseError::new(format!("bad direction: {}", direction))),
        };
        Ok(command)
    }
}

struct Commands {
    lines: Lines<BufReader<File>>,
}

impl Commands {
    fn load<P: AsRef<Path>>(input: P) -> Result<Commands, io::Error> {
        let lines = BufReader::new(File::open(input)?).lines();
        Ok(Commands { lines })
    }
}

impl Iterator for Commands {
    type Item = Command;

    fn next(&mut self) -> Option<Command> {
        self.lines
            .next()
            .and_then(|result| result.ok())
            .and_then(|line| line.parse().ok())
    }
}

pub mod part1 {
    use super::{Axis, Command};

    pub fn solve<I>(commands: I) -> i32
    where
        I: Iterator<Item = Command>,
    {
        let (mut hpos, mut depth) = (0, 0);
        for Command(axis, offset) in commands {
            match axis {
                Axis::Horizontal => hpos += offset,
                Axis::Vertical => depth += offset,
            }
        }
        hpos * depth
    }

    #[cfg(test)]
    mod tests {
        use super::super::Commands;
        use super::solve;

        #[test]
        fn test_solve() {
            let commands = Commands::load("tests/day2/sample").unwrap();
            assert_eq!(150, solve(commands));
        }
    }
}

pub mod part2 {
    use super::{Axis, Command};

    pub fn solve<I>(commands: I) -> i32
    where
        I: Iterator<Item = Command>,
    {
        let (mut aim, mut hpos, mut depth) = (0, 0, 0);
        for Command(axis, offset) in commands {
            match axis {
                Axis::Horizontal => {
                    hpos += offset;
                    depth += aim * offset;
                }
                Axis::Vertical => aim += offset,
            }
        }
        hpos * depth
    }

    #[cfg(test)]
    mod tests {
        use super::super::Commands;
        use super::solve;

        #[test]
        fn test_solve() {
            let commands = Commands::load("tests/day2/sample").unwrap();
            assert_eq!(900, solve(commands));
        }
    }
}

fn main() {
    let input = "tests/day2/input";
    let commands: Vec<_> = Commands::load(input)
        .unwrap_or_else(|err| {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(3);
        })
        .collect();
    println!("{}", part1::solve(commands.iter().cloned()));
    println!("{}", part2::solve(commands.into_iter()));
}
