use crate::row::Row;

pub fn solve(text: &str) -> i32 {
    text.lines()
        .map(Row::from_line)
        .map(Row::backward)
        .map(Row::solve)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 2);
    }
}
