use advent2020::day7::*;

fn main() {
    let input = "tests/day7/input";
    let color = ("shiny".to_owned(), "gold".to_owned());
    let answer1 = match part1::solve(input, &color) {
        Ok(answer) => answer,
        Err(err) => {
            eprintln!("error: {}: {}", input, err);
            std::process::exit(1);
        }
    };
    println!("{}", answer1);
}
