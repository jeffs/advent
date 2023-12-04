use crate::card::Card;

pub fn solve(text: &str) -> usize {
    text.lines()
        .map(|line| 1 << Card::from_line(line).count >> 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 13);
    }
}
