fn main() {
    let text = include_str!("input.txt");
    println!("{}", day6::part1::solve(text));
    println!("{}", day6::part2::solve(text));
}
