use advent2020::day19::{part1, part2};
use std::fs;

fn main() {
    let input = "tests/day19/input";
    let text = match fs::read_to_string(input) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(3);
        }
    };
    match part1::solve(&text) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    }
    match part2::solve(&text) {
        Ok(answer) => println!("{}", answer),
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(2);
        }
    }
}
