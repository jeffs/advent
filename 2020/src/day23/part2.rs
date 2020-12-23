use super::circle::Circle;

pub fn solve(digits: u64) -> u64 {
    Circle::solve2(digits, 999_999, 10_000_000)
}
