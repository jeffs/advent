use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

type BoxedError = Box<dyn Error>;

#[derive(Debug)]
struct StaticError {
    what: &'static str,
}

impl StaticError {
    fn boxed(what: &'static str) -> BoxedError {
        Box::new(StaticError { what })
    }
}

impl fmt::Display for StaticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.what)
    }
}

impl Error for StaticError {}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(self) -> u64 {
        self as u64 + 1
    }
}

struct Round {
    player1: Shape,
    player2: Shape,
}

impl Round {
    fn score(&self) -> u64 {
        let shape_score = self.player2.score();
        let outcome_score = match (self.player1, self.player2) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,
        };
        shape_score + outcome_score
    }
}

impl FromStr for Round {
    type Err = BoxedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() < 3
            || bytes.len() > 4
            || (bytes[1] != b' ')
            || (bytes.len() == 4 && bytes[3] != b'\n')
        {
            return Err(StaticError::boxed("bad Round"));
        }
        let player1 = match bytes[0] {
            b'A' => Shape::Rock,
            b'B' => Shape::Paper,
            b'C' => Shape::Scissors,
            _ => {
                return Err(StaticError::boxed("bad Round::player1 value"));
            }
        };
        let player2 = match bytes[2] {
            b'X' => Shape::Rock,
            b'Y' => Shape::Paper,
            b'Z' => Shape::Scissors,
            _ => {
                return Err(StaticError::boxed("bad Round::player2 value"));
            }
        };
        Ok(Round { player1, player2 })
    }
}

pub struct Puzzle {
    path: PathBuf,
}

impl Puzzle {
    fn from_path(path: impl Into<PathBuf>) -> Puzzle {
        Puzzle { path: path.into() }
    }

    fn rounds(&self) -> Result<impl Iterator<Item = Result<Round, BoxedError>>, BoxedError> {
        let file = File::open(&self.path)?;
        let lines = BufReader::new(file).lines();
        Ok(lines.map(|res| res?.parse()))
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(puzzle: Puzzle) -> Result<u64, BoxedError> {
        let rounds = puzzle.rounds()?;
        let scores = rounds.map(|res| res.map(|round| round.score()));
        let mut sum = 0;
        for score in scores {
            sum += score?;
        }
        Ok(sum)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let puzzle = Puzzle::from_path("tests/day2/sample");
            let answer = solve(puzzle).expect("part 1: sample should be solvable");
            assert_eq!(15, answer);
        }

        #[test]
        fn test_solve_input() {
            let puzzle = Puzzle::from_path("tests/day2/input");
            let answer = solve(puzzle).expect("part 1: input should be solvable");
            assert_eq!(11767, answer);
        }
    }
}

fn main() {
    let input = "tests/day2/input";
    for solve in [part1::solve] {
        let puzzle = Puzzle::from_path(input);
        let answer = solve(puzzle).unwrap_or_else(|err| {
            eprintln!("error: {err}");
            exit(1);
        });
        println!("{answer}");
    }
}
