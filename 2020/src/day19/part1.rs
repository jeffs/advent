use crate::error::ParseError;
use std::fs;

pub fn solve(_text: &str) -> Result<usize, ParseError> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_sample1() {
        let text = fs::read_to_string("tests/day19/sample1").unwrap();
        assert_eq!(2, solve(&text).unwrap());
    }
}
