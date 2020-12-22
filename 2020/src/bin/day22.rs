use advent2020::day22::{deck, part1};
use std::fs;

fn main() {
    let input_path = "tests/day22/input";
    let text = fs::read_to_string(input_path).unwrap();
    let decks = deck::parse_both(text).unwrap();
    println!("{}", part1::solve(decks));
}
