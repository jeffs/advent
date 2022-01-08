mod cuboid;
mod puzzle;
mod range;
mod state;
mod step;

use self::cuboid::Cuboid;
use self::puzzle::Puzzle;
use self::state::State;

use std::collections::HashSet;

pub mod part1 {
    use super::*;

    pub fn solve(puzzle: &Puzzle) -> usize {
        let mut reactor = HashSet::new(); // which cubes are on
        for step in &puzzle.steps {
            let step = step.constrain(50);
            for cube in step.cubes() {
                match step.state {
                    State::Off => reactor.remove(&cube),
                    State::On => reactor.insert(cube),
                };
            }
        }
        reactor.len()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day22/sample").unwrap();
            assert_eq!(590784, solve(&puzzle));
        }
    }
}

pub mod part2 {
    use super::*;

    pub fn solve(puzzle: &Puzzle) -> usize {
        let mut on: Vec<Cuboid> = Vec::new();
        for step in puzzle.steps.iter() {
            // Replace each existing cuboid overlapped by the new one (from the
            // current step) with a set of non-overlapped sub-cuboids.
            on = on
                .into_iter()
                .flat_map(|block| block.minus(&step.block))
                .collect();
            if let State::On = step.state {
                on.push(step.block.clone());
            }
        }
        on.into_iter().map(|cuboid| cuboid.volume()).sum()
    }

    #[cfg(test)]
    mod tests {
        use super::{solve, Puzzle};

        #[test]
        fn test_solve() {
            let puzzle = Puzzle::from_file("tests/day22/sample2").unwrap();
            assert_eq!(2758514936282235, solve(&puzzle));
        }
    }
}

fn main() {
    let input = "tests/day22/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&puzzle));
}
