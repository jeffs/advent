use advent2021::{EmptyFile, NoSolution, ParseError};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::num::ParseIntError;
use std::path::Path;

#[derive(Clone, Copy, Debug, Default)]
struct Cell {
    value: u64,
    stamp: bool,
}

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;

type BoardRow = [Cell; BOARD_WIDTH];

#[derive(Debug)]
struct Board([BoardRow; BOARD_HEIGHT]);

impl Board {
    fn parse_row(line: &str, row: &mut BoardRow) -> Result<(), ParseError> {
        let mut fields = line.split_ascii_whitespace();
        for cell in row.iter_mut() {
            cell.value = fields
                .next()
                .ok_or_else(|| ParseError::new(format!("too few values in row: {}", line)))?
                .parse()?;
        }
        if fields.next().is_some() {
            return Err(ParseError::new(format!("too many values in row: {}", line)));
        }
        Ok(())
    }

    fn stamp(&mut self, value: u64) {
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                let cell = &mut self.0[i][j];
                cell.stamp |= cell.value == value;
            }
        }
    }

    fn has_won(&self) -> bool {
        self.0.iter().any(|row| row.iter().all(|cell| cell.stamp))
            || (0..BOARD_WIDTH).any(|j| self.0.iter().all(|row| row[j].stamp))
    }

    fn score(&self) -> u64 {
        self.0
            .iter()
            .flat_map(|row| {
                row.iter()
                    .filter_map(|cell| if cell.stamp { None } else { Some(cell.value) })
            })
            .sum()
    }

    fn reset(&mut self) {
        for i in 0..BOARD_HEIGHT {
            for j in 0..BOARD_WIDTH {
                self.0[i][j].stamp = false;
            }
        }
    }

    fn try_parse<E: 'static, I>(lines: &mut I) -> Result<Option<Board>, Box<dyn Error>>
    where
        E: Error,
        I: Iterator<Item = Result<String, E>>,
    {
        Ok(if let Some(line) = lines.next() {
            let mut rows = [[Cell::default(); BOARD_WIDTH]; BOARD_HEIGHT];
            Board::parse_row(&line?, &mut rows[0])?;
            for row in rows.iter_mut().skip(1) {
                let line = lines
                    .next()
                    .ok_or_else(|| ParseError::new("too few rows"))?;
                Board::parse_row(&line?, row)?;
            }
            Some(Board(rows))
        } else {
            None
        })
    }
}

#[derive(Debug)]
pub struct Game {
    values: Vec<u64>,
    boards: Vec<Board>,
}

impl Game {
    fn reset(&mut self) {
        for board in &mut self.boards {
            board.reset();
        }
    }
}

fn parse_values(line: &str) -> Result<Vec<u64>, ParseIntError> {
    let mut values = Vec::new();
    for field in line.split(',') {
        values.push(field.parse()?);
    }
    Ok(values)
}

fn skip_empty_line<E: 'static, I>(lines: &mut I) -> Result<(), Box<dyn Error>>
where
    E: Error,
    I: Iterator<Item = Result<String, E>>,
{
    if let Some(line) = lines.next() {
        let line = line?;
        if !line.is_empty() {
            return Err(Box::new(ParseError::new(format!(
                "unexpected non-empty line: {}",
                line
            ))));
        }
    }
    Ok(())
}

fn parse_boards<E: 'static, I>(mut lines: I) -> Result<Vec<Board>, Box<dyn Error>>
where
    E: Error,
    I: Iterator<Item = Result<String, E>>,
{
    let mut boards = Vec::new();
    while let Some(board) = Board::try_parse(&mut lines)? {
        boards.push(board);
        skip_empty_line(&mut lines)?;
    }
    Ok(boards)
}

fn load_game<P: AsRef<Path>>(input: P) -> Result<Game, Box<dyn Error>> {
    let mut lines = BufReader::new(File::open(&input)?).lines();
    let values = parse_values(&lines.next().ok_or_else(|| EmptyFile::new(input))??)?;
    skip_empty_line(&mut lines)?;
    let boards = parse_boards(lines)?;
    Ok(Game { values, boards })
}

pub mod part1 {
    use super::*;

    pub fn solve(game: &mut Game) -> Result<u64, NoSolution> {
        for &value in &game.values {
            for board in game.boards.iter_mut() {
                board.stamp(value);
                if board.has_won() {
                    return Ok(board.score() * value);
                }
            }
        }
        Err(NoSolution)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(
                Ok(4512),
                solve(&mut load_game("tests/day4/sample").unwrap())
            );
        }
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(game: &mut Game) -> Result<u64, NoSolution> {
        let mut last = None;
        for &value in &game.values {
            for board in game.boards.iter_mut() {
                if board.has_won() {
                    continue;
                }
                board.stamp(value);
                if board.has_won() {
                    last.replace(board.score() * value);
                }
            }
        }
        last.ok_or(NoSolution)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_solve() {
            assert_eq!(
                Ok(1924),
                solve(&mut load_game("tests/day4/sample").unwrap())
            );
        }
    }
}

fn main() {
    let input = "tests/day4/input";
    let mut game = load_game("tests/day4/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    match part1::solve(&mut game) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
    game.reset();
    match part2::solve(&mut game) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
}
