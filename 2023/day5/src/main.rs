fn main() {
    let text = include_str!("input.txt");
    println!("{}", day5::part1::solve(text));
    println!("{}", day5::part2::solve(text));
}
