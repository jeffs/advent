use std::fs::File;
use std::io::{self, BufRead as _, BufReader};
use std::path::Path;

mod day13 {
    use super::*;

    pub fn load_lines<P>(input: P) -> Result<Vec<Vec<u8>>, io::Error>
    where
        P: AsRef<Path>,
    {
        let mut result = Vec::new();
        for line in BufReader::new(File::open(input)?).lines() {
            result.push(line?.into_bytes());
        }
        Ok(result)
    }

    pub mod part1 {
        pub fn solve(_lines: &[Vec<u8>]) -> usize {
            todo!()
        }

        #[cfg(test)]
        mod tests {
            use super::super::load_lines;
            use super::solve;

            #[test]
            fn test_solve() {
                let lines = load_lines("tests/day13/sample").unwrap();
                assert_eq!(0xDeadBeef, solve(&lines));
            }
        }
    }
}

fn main() {
    let input = "tests/day13/input";
    let lines = day13::load_lines(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", day13::part1::solve(&lines));
}
