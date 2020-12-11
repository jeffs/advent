use advent2020::day11::Grid;
use std::error::Error;
use std::mem;
use std::path::Path;

fn solve_part1<P>(input: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut old = Grid::from_file(input)?;
    let mut new = Grid::with_size(old.size());
    loop {
        old.next1(&mut new);
        if old == new {
            return Ok(old.pop_count());
        }
        mem::swap(&mut old, &mut new);
    }
}

fn solve_part2<P>(input: P) -> Result<usize, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let mut old = Grid::from_file(input)?;
    let mut new = Grid::with_size(old.size());
    loop {
        old.next2(&mut new);
        if old == new {
            return Ok(old.pop_count());
        }
        mem::swap(&mut old, &mut new);
    }
}

fn main() {
    let input = "tests/day11/input";
    println!("{}", solve_part1(input).unwrap());
    println!("{}", solve_part2(input).unwrap());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve1_sample1() {
        let input = "tests/day11/sample1";
        assert_eq!(37, solve_part1(input).unwrap());
    }

    #[test]
    fn solve2_sample1() {
        let input = "tests/day11/sample1";
        assert_eq!(26, solve_part2(input).unwrap());
    }
}
