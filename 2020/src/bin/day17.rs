use advent2020::day17::{part1, part2};

fn main() {
    let input_path = "tests/day17/input";
    println!("{}", part1::solve(input_path).unwrap());
    println!("{}", part2::solve(input_path).unwrap());
}
