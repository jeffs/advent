fn main() {
    let text = include_str!("input.txt");
    println!("{}", day3::part1::solve(text));
    println!("{}", day3::part2::solve(text));
}
