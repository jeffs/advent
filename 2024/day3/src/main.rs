use day3::{part1, part2};

fn main() {
    let input = include_str!("input");
    println!("{}", part1::solve(input));
    println!("{}", part2::solve(input));
}
