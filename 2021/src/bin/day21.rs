#![allow(dead_code, unused_variables)]

use advent2021::ParseError;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

fn wrap_add(i: usize, d: usize, n: usize) -> usize {
    assert!(i > 0);
    (i - 1 + d) % n + 1
}

fn wrap_inc(i: usize, n: usize) -> usize {
    wrap_add(i, 1, n)
}

struct Die {
    next: usize,
}

impl Die {
    pub fn new() -> Die {
        Die { next: 1 }
    }

    pub fn roll(&mut self) -> usize {
        let next = wrap_inc(self.next, 100);
        std::mem::replace(&mut self.next, next)
    }

    pub fn roll_sum(&mut self, n: usize) -> usize {
        (0..n).map(|_| self.roll()).sum()
    }
}

pub struct Puzzle {
    starts: [usize; 2],
}

impl Puzzle {
    fn from_file<P>(input: P) -> Result<Puzzle, ParseError>
    where
        P: AsRef<Path>,
    {
        let input = input.as_ref();
        let mut starts = [0; 2];
        let mut output = starts.iter_mut();
        for line in BufReader::new(File::open(input)?).lines() {
            let start = line?
                .rsplit_once(' ')
                .ok_or_else(|| ParseError::in_file(input, "bad player; failed to split"))?
                .1
                .parse()?;
            *output
                .next()
                .ok_or_else(|| ParseError::in_file(input, "not enough players"))? = start;
        }
        if output.next().is_some() {
            return Err(ParseError::in_file(input, "too many players"));
        }
        Ok(Puzzle { starts })
    }
}

pub mod part1 {
    use super::*;

    struct Player {
        position: usize,
        score: usize,
    }

    impl Player {
        fn advance(&mut self, distance: usize) {
            self.position = wrap_add(self.position, distance, 10);
            self.score += self.position;
        }
    }

    pub fn solve(puzzle: &Puzzle) -> usize {
        let mut die = Die::new();
        let mut players = [
            Player {
                position: puzzle.starts[0],
                score: 0,
            },
            Player {
                position: puzzle.starts[1],
                score: 0,
            },
        ];
        players[0].advance(die.roll_sum(3));
        let mut i = 1; // number of turns taken so far
        while players[(i - 1) % 2].score < 1000 {
            players[i % 2].advance(die.roll_sum(3));
            i += 1;
        }
        let score = players[i % 2].score;
        let rolls = i * 3;
        score * rolls
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            assert_eq!(739785, solve(&Puzzle { starts: [4, 8] }));
        }
    }
}

fn main() {
    let input = "tests/day21/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&puzzle));
}
