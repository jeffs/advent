use super::Grid;
use std::error::Error;
use std::mem;
use std::path::Path;

pub fn solve<P>(input: P) -> Result<usize, Box<dyn Error>>
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample1() {
        let input = "tests/day11/sample1";
        assert_eq!(37, solve(input).unwrap());
    }
}
