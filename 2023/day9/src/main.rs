fn main() {
    let text = include_str!("input.txt");
    println!("{}", day9::part1::solve(text));
    println!("{}", day9::part2::solve(text));
}
