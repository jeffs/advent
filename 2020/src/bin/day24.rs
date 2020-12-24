use advent2020::day24::{part1, part2};
use std::fs;

fn main() {
    let input_path = "tests/day24/input";
    let text = fs::read_to_string(input_path).unwrap();
    println!("{}", part1::solve(&text).unwrap());
    println!("{}", part2::solve(&text).unwrap());
}
