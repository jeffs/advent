fn main() {
    let text = include_str!("input.txt");
    println!("{}", day11::part1::solve(text));
    println!("{}", day11::part2::solve(text));
}
