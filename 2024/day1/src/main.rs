use day1::{part1, Input};

fn main() {
    let s = include_str!("input");
    let Input(xs, ys) = s.parse().unwrap();
    println!("{}", part1::distance(xs.iter().cloned(), ys));
}
