fn parse_digit(line: &str) -> Option<u32> {
    for (value, word) in [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ]
    .into_iter()
    .enumerate()
    {
        let digit = value.to_string();
        if line.starts_with(&digit) || line.starts_with(word) {
            return Some(value as u32);
        }
    }
    None
}

/// Returns the two-digit value represented by the first and last digits in the
/// specified line, be they represented as numerals (0, 1, 2...) or words (zero,
/// one, two...).
/// ```
/// assert_eq!(day1::part2::parse2d("aoneandatwo"), 12);
/// assert_eq!(day1::part2::parse2d("foothreebar"), 33);
/// ```
pub fn parse2d(line: &str) -> u32 {
    const BASE: u32 = 10;
    let digits: Vec<u32> = (0..line.len())
        .filter_map(|i| parse_digit(&line[i..]))
        .collect();
    let first = digits.first().cloned().unwrap_or_default();
    let last = digits.last().cloned().unwrap_or_default();
    first * BASE + last
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
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(solve(text), 281);
    }
}
