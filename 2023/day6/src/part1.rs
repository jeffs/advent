use crate::count::count;

fn parse_line(line: &str) -> impl Iterator<Item = i64> + '_ {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().expect("number"))
}

pub fn solve(text: &str) -> i64 {
    let (times, distances) = text.split_once('\n').expect("two input lines");
    let (times, distances) = (parse_line(times), parse_line(distances));
    times
        .zip(distances)
        .map(|(time, distance)| count(time, distance))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 288);
    }
}
