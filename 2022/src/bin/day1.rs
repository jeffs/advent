use advent2022::BoxedError;
use std::fs::File;
use std::io::{BufRead as _, BufReader, Lines};
use std::path::Path;
use std::process::exit;

// An iterator over elves' total calorie counts.
struct Elves {
    lines: Lines<BufReader<File>>,
}

impl Elves {
    fn from_path(path: impl AsRef<Path>) -> Result<Elves, BoxedError> {
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines();
        Ok(Elves { lines })
    }

    /// Returns the number from the next line, or None if the line is blank or
    /// there is no next line.  Returns an error if reading or parsing fails.
    fn parse_next_line(&mut self) -> Option<Result<u64, BoxedError>> {
        let Some(result) = self.lines.next() else {
            return None;
        };
        let line = match result {
            Ok(line) => line,
            Err(err) => return Some(Err(Box::new(err))),
        };
        if line.is_empty() {
            return None;
        }
        match line.parse() {
            Ok(number) => Some(Ok(number)),
            Err(err) => Some(Err(Box::new(err))),
        }
    }
}

impl Iterator for Elves {
    type Item = Result<u64, BoxedError>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.parse_next_line();
        let Some(Ok(mut sum)) = first else {
            return first;
        };
        while let Some(result) = self.parse_next_line() {
            let Ok(number) = result else {
                return Some(result);
            };
            sum += number;
        }
        Some(Ok(sum))
    }
}

pub struct Puzzle {
    elves: Elves,
}

impl Puzzle {
    fn from_path(path: impl AsRef<Path>) -> Result<Puzzle, BoxedError> {
        Ok(Puzzle {
            elves: Elves::from_path(path)?,
        })
    }
}

pub mod part1 {
    use super::*;

    pub fn solve(puzzle: Puzzle) -> Result<u64, BoxedError> {
        let mut elves = puzzle.elves;
        let mut max = elves.next().expect("there should be at least one elf")?;
        for elf in elves {
            max = max.max(elf?);
        }
        Ok(max)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let puzzle = Puzzle::from_path("tests/day1/sample").unwrap();
            let answer = solve(puzzle).expect("part 1: sample should be solvable");
            assert_eq!(24000, answer);
        }

        #[test]
        fn test_solve_input() {
            let puzzle = Puzzle::from_path("tests/day1/input").unwrap();
            let answer = solve(puzzle).expect("part 1: input should be solvable");
            assert_eq!(67658, answer);
        }
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(puzzle: Puzzle) -> Result<u64, BoxedError> {
        let mut top3 = [0; 3];  // sorted in ascending order
        for elf in puzzle.elves {
            let elf = elf?;
            if elf > top3[2] {
                top3[0] = top3[1];
                top3[1] = top3[2];
                top3[2] = elf;
            } else if elf > top3[1] {
                top3[0] = top3[1];
                top3[1] = elf;
            } else if elf > top3[0] {
                top3[0] = elf;
            }
        }
        Ok(top3[0] + top3[1] + top3[2])
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve_sample() {
            let puzzle = Puzzle::from_path("tests/day1/sample").unwrap();
            let answer = solve(puzzle).expect("part 2: sample should be solvable");
            assert_eq!(45000, answer);
        }

        #[test]
        fn test_solve_input() {
            let puzzle = Puzzle::from_path("tests/day1/input").unwrap();
            let answer = solve(puzzle).expect("part 2: input should be solvable");
            assert_eq!(200158, answer);
        }
    }
}

fn main() {
    let input = "tests/day1/input";
    for solve in [part1::solve, part2::solve] {
        let puzzle = Puzzle::from_path(input).unwrap_or_else(|err| {
            eprintln!("error: {}: {}", input, err);
            exit(3);
        });
        let answer = solve(puzzle).unwrap_or_else(|err| {
            eprintln!("error: {err}");
            exit(1);
        });
        println!("{answer}");
    }
}
