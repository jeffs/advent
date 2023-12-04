fn main() {
    let text = include_str!("input.txt");
    println!("{}", day4::part1::solve(text));
    println!("{}", day4::part2::solve(text));
}
