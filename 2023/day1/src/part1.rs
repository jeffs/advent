/// Returns the two-digit value represented by the first and last numeric digits
/// in the specified line.
/// ```
/// assert_eq!(day1::part2::parse2d("a4b2c"), 42);
/// assert_eq!(day1::part2::parse2d("ab7cd"), 77);
/// ```
pub fn parse2d(line: &str) -> u32 {
    const BASE: u32 = 10;
    let first = line.chars().find_map(|c| c.to_digit(BASE));
    let last = line.chars().rev().find_map(|c| c.to_digit(BASE));
    first.unwrap_or_default() * BASE + last.unwrap_or_default()
}

pub fn solve(text: &str) -> u32 {
    text.lines().map(parse2d).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let text = r#"
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "#;
        assert_eq!(solve(text), 142);
    }
}
