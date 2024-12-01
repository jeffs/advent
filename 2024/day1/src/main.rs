use day1::part1;

fn main() {
    let s = include_str!("part1/input");
    println!("{}", part1::solve(&s).unwrap());
}
