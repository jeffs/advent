#![allow(dead_code, unused_imports, unused_variables)]

use advent2021::{EmptyFile, NoSolution, ParseError};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader, Lines};
use std::num::ParseIntError;
use std::path::Path;

mod day4 {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    struct Cell {
        value: u32,
        stamp: bool,
    }

    impl Default for Cell {
        fn default() -> Cell {
            Cell {
                value: 0,
                stamp: false,
            }
        }
    }

    const BOARD_WIDTH: usize = 5;
    const BOARD_HEIGHT: usize = 5;

    type BoardRow = [Cell; BOARD_WIDTH];

    #[derive(Debug)]
    struct Board([BoardRow; BOARD_HEIGHT]);

    impl Board {
        fn parse_row(line: &str, row: &mut BoardRow) -> Result<(), ParseError> {
            let mut fields = line.split_ascii_whitespace();
            for j in 0..BOARD_WIDTH {
                row[j].value = fields
                    .next()
                    .ok_or_else(|| ParseError::new(format!("too few values in row: {}", line)))?
                    .parse()?;
            }
            if fields.next().is_some() {
                return Err(ParseError::new(format!("too many values in row: {}", line)));
            }
            Ok(())
        }

        fn try_parse<E: 'static, I>(lines: &mut I) -> Result<Option<Board>, Box<dyn Error>>
        where
            E: Error,
            I: Iterator<Item = Result<String, E>>,
        {
            Ok(if let Some(line) = lines.next() {
                let mut rows = [[Cell::default(); BOARD_WIDTH]; BOARD_HEIGHT];
                Board::parse_row(&line?, &mut rows[0])?;
                for i in 1..BOARD_HEIGHT {
                    let line = lines
                        .next()
                        .ok_or_else(|| ParseError::new("too few rows"))?;
                    Board::parse_row(&line?, &mut rows[i])?;
                }
                Some(Board(rows))
            } else {
                None
            })
        }
    }

    #[derive(Debug)]
    pub struct Game {
        values: Vec<u32>,
        boards: Vec<Board>,
    }

    fn parse_values(line: &str) -> Result<Vec<u32>, ParseIntError> {
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

    pub fn load_game<P: AsRef<Path>>(input: P) -> Result<Game, Box<dyn Error>> {
        let mut lines = BufReader::new(File::open(&input)?).lines();
        let values = parse_values(&lines.next().ok_or_else(|| EmptyFile::new(input))??)?;
        skip_empty_line(&mut lines)?;
        let boards = parse_boards(lines)?;
        Ok(Game { values, boards })
    }

    pub mod part1 {
        use super::*;

        pub fn solve(Game { values, boards }: Game) -> Result<u32, NoSolution> {
            println!("{:#?}", boards);
            todo!()
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                assert_eq!(Ok(4512), solve(load_game("tests/day4/sample").unwrap()));
            }
        }
    }
}

fn main() {
    let input = "tests/day4/input";
    let game = day4::load_game("tests/day4/input").unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    match day4::part1::solve(game) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
}
