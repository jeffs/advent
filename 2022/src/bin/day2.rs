use advent2022::{BoxedError, StaticError};
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::{Path, PathBuf};
use std::process::exit;

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_player1(byte: u8) -> Result<Shape, BoxedError> {
        match byte {
            b'A' => Ok(Shape::Rock),
            b'B' => Ok(Shape::Paper),
            b'C' => Ok(Shape::Scissors),
            _ => Err(StaticError::boxed("bad Round::player1 value")),
        }
    }

    fn score(self) -> u64 {
        self as u64 + 1
    }
}

/// Returns bytes from the specified line representing the moves of both
/// players.
fn to_bytes(line: &str) -> Result<(u8, u8), BoxedError> {
    let bytes = line.as_bytes();
    if bytes.len() < 3
        || bytes.len() > 4
        || (bytes[1] != b' ')
        || (bytes.len() == 4 && bytes[3] != b'\n')
    {
        return Err(StaticError::boxed("bad Round"));
    }
    Ok((bytes[0], bytes[2]))
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

pub struct Puzzle {
    path: PathBuf,
}

impl Puzzle {
    fn from_path(path: impl Into<PathBuf>) -> Puzzle {
        Puzzle { path: path.into() }
    }
}

pub mod part1 {
    use super::*;

    fn round_from_str(s: &str) -> Result<Round, BoxedError> {
        let (byte1, byte2) = to_bytes(s)?;
        let player1 = Shape::from_player1(byte1)?;
        let player2 = match byte2 {
            b'X' => Shape::Rock,
            b'Y' => Shape::Paper,
            b'Z' => Shape::Scissors,
            _ => {
                return Err(StaticError::boxed("bad Round::player2 value"));
            }
        };
        Ok(Round { player1, player2 })
    }

    fn rounds_from_path(
        path: &Path,
    ) -> Result<impl Iterator<Item = Result<Round, BoxedError>>, BoxedError> {
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines();
        Ok(lines.map(|res| round_from_str(&res?)))
    }

    pub fn solve(puzzle: Puzzle) -> Result<u64, BoxedError> {
        let rounds = rounds_from_path(&puzzle.path)?;
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
            let answer = solve(puzzle).expect("sample should be solvable");
            assert_eq!(15, answer);
        }

        #[test]
        fn test_solve_input() {
            let puzzle = Puzzle::from_path("tests/day2/input");
            let answer = solve(puzzle).expect("input should be solvable");
            assert_eq!(11767, answer);
        }
    }
}

pub mod part2 {
    use super::*;

    fn round_from_str(s: &str) -> Result<Round, BoxedError> {
        let (byte1, byte2) = to_bytes(s)?;
        let player1 = Shape::from_player1(byte1)?;
        let player2 = match (player1, byte2) {
            // Lose
            (Shape::Rock, b'X') => Shape::Scissors,
            (Shape::Paper, b'X') => Shape::Rock,
            (Shape::Scissors, b'X') => Shape::Paper,
            // Draw
            (_, b'Y') => player1,
            // Win
            // Lose
            (Shape::Rock, b'Z') => Shape::Paper,
            (Shape::Paper, b'Z') => Shape::Scissors,
            (Shape::Scissors, b'Z') => Shape::Rock,
            _ => {
                return Err(StaticError::boxed("bad Round::player2 value"));
            }
        };
        Ok(Round { player1, player2 })
    }

    fn rounds_from_path(
        path: &Path,
    ) -> Result<impl Iterator<Item = Result<Round, BoxedError>>, BoxedError> {
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines();
        Ok(lines.map(|res| round_from_str(&res?)))
    }

    pub fn solve(puzzle: Puzzle) -> Result<u64, BoxedError> {
        let rounds = rounds_from_path(&puzzle.path)?;
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
            let answer = solve(puzzle).expect("sample should be solvable");
            assert_eq!(12, answer);
        }

        #[test]
        fn test_solve_input() {
            let puzzle = Puzzle::from_path("tests/day2/input");
            let answer = solve(puzzle).expect("input should be solvable");
            assert_eq!(13886, answer);
        }
    }
}

fn main() {
    let input = "tests/day2/input";
    for solve in [part1::solve, part2::solve] {
        let puzzle = Puzzle::from_path(input);
        let answer = solve(puzzle).unwrap_or_else(|err| {
            eprintln!("error: {err}");
            exit(1);
        });
        println!("{answer}");
    }
}
