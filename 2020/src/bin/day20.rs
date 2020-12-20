use advent2020::day20::part1;
use std::fs;

fn main() {
    let input = "tests/day20/input";
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
}
