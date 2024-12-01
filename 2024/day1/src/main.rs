use day1::{part1::distance, part2::similarity, Input};

fn main() {
    let s = include_str!("input");
    let Input(xs, ys) = s.parse().unwrap();
    println!("{}", distance(xs.iter().cloned(), ys.iter().cloned()));
    println!("{}", similarity(xs, ys));
}
