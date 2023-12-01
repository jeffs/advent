fn main() {
    let text = include_str!("input.txt");
    println!("{}", day1::part1::solve(text));
    println!("{}", day1::part2::solve(text));
}
