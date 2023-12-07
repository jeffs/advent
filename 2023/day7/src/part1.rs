use crate::line::{IntoWinnings, Line};

pub fn solve(text: &str) -> i64 {
    text.lines().map(Line::parse).into_winnings()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 6440);
    }
}
