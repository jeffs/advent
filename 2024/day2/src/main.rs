use day2::{part1, part2};

fn main() {
    let input = include_str!("input").parse().expect("input");
    println!("{}", part1::count_safe(&input));
    println!("{}", part2::count_safe(&input));
}
