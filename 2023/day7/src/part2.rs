use crate::line::{IntoWinnings, Line};

pub fn solve(text: &str) -> i64 {
    text.lines()
        .map(Line::parse)
        .map(Line::with_jokers)
        .into_winnings()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 5905);
    }
}
