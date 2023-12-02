fn main() {
    let text = include_str!("input.txt");
    println!("{}", day2::part1::solve(text));
    println!("{}", day2::part2::solve(text));
}
