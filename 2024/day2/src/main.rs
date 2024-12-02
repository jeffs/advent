use day2::part1;

fn main() {
    let input = include_str!("input").parse().expect("input");
    println!("{}", part1::count_safe(&input));
}
