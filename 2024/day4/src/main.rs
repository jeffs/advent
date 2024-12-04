use day4::part1;

fn main() -> part1::Result<()> {
    let input = include_str!("input");
    println!("{}", part1::solve(input)?);
    Ok(())
}
