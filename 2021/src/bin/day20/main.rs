mod algorithm;
mod image;
mod puzzle;

use crate::puzzle::Puzzle;

pub mod part1 {
    use super::*;

    pub fn solve(puzzle: &Puzzle) -> Result<usize, &'static str> {
        let algo = &puzzle.algo;
        let image = algo.enhance(&puzzle.image);
        let image = algo.enhance(&image);
        image.count_lights()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day20/sample").unwrap();
            assert_eq!(35, solve(&puzzle).unwrap());
        }
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(puzzle: &Puzzle) -> Result<usize, &'static str> {
        let algo = &puzzle.algo;
        let mut image = algo.enhance(&puzzle.image);
        for _ in 0..49 {
            image = algo.enhance(&image);
        }
        image.count_lights()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day20/sample").unwrap();
            assert_eq!(3351, solve(&puzzle).unwrap());
        }
    }
}

fn main() {
    let input = "tests/day20/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    match part1::solve(&puzzle) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    }
    match part2::solve(&puzzle) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}", err);
            std::process::exit(1);
        }
    }
}
