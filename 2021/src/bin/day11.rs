use advent2021::ParseError;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead as _, BufReader};
use std::path::Path;

mod day11 {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct Octopus(u8);

    #[derive(Clone, Copy)]
    pub struct Grid([[Octopus; 10]; 10]);

    impl Grid {
        pub fn from_file<P>(input: P) -> Result<Self, Box<dyn Error>>
        where
            P: AsRef<Path> + Clone,
        {
            let mut rows = [[Octopus(0); 10]; 10];
            let mut lines = BufReader::new(File::open(&input)?).lines();
            for row in rows.iter_mut() {
                let line = lines
                    .next()
                    .ok_or_else(|| ParseError::in_file(input.clone(), "unexpected EOF"))??;
                for (cell, digit) in row.iter_mut().zip(line.bytes()) {
                    *cell = Octopus(digit - b'0');
                }
            }
            Ok(Grid(rows))
        }
    }

    pub mod part1 {
        use super::*;

        pub fn solve(mut _grid: Grid) -> u64 {
            todo!()
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_solve() {
                let lines = Grid::from_file("tests/day11/sample").unwrap();
                assert_eq!(1656, solve(&lines));
            }
        }
    }
}

fn main() {
    let input = "tests/day11/input";
    let lines = day11::Grid::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day11::part1::solve(lines));
}
