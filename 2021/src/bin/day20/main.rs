mod algorithm;
mod image;
mod puzzle;

use crate::puzzle::Puzzle;

pub mod part1 {
    use super::*;

    pub fn solve(puzzle: &Puzzle) -> usize {
        let algo = &puzzle.algo;
        let image = algo.enhance(&algo.enhance(&puzzle.image));
        image.len()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day20/sample").unwrap();
            assert_eq!(35, solve(&puzzle));
        }
    }
}

fn main() {
    let input = "tests/day20/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&puzzle));
}
