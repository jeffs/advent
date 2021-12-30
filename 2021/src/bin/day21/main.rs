mod part1;
mod part2;
mod player;
mod puzzle;
mod wrap;

use crate::puzzle::Puzzle;

fn main() {
    let input = "tests/day21/input";
    let puzzle = Puzzle::from_file(input).unwrap_or_else(|err| {
        eprintln!("error: {}: {}", input, err);
        std::process::exit(3);
    });
    println!("{}", part1::solve(&puzzle));
    println!("{}", part2::solve(&puzzle));
}
