fn main() {
    let text = include_str!("input.txt");
    println!("{}", day7::part1::solve(text));
    println!("{}", day7::part2::solve(text));
}
