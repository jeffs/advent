fn main() {
    let text = include_str!("input.txt");
    println!("{}", day8::part1::solve(text));
    println!("{}", day8::part2::solve(text));
}
