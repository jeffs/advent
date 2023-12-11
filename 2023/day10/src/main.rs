fn main() {
    let text = include_str!("input.txt");
    println!("{}", day10::part1::solve(text));
    println!("{}", day10::part2::solve(text));
}
