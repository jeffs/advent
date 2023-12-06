use crate::count::count;

pub fn solve(text: &str) -> i64 {
    let (times, distances) = text.split_once('\n').expect("two input lines");
    let times = times
        .split_ascii_whitespace()
        .skip(1)
        .map(|t| t.parse::<i64>().expect("times to be numbers"));
    let distances = distances
        .split_ascii_whitespace()
        .skip(1)
        .map(|d| d.parse::<i64>().expect("distances to be numbers"));
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
