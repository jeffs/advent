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
        .map(|(time, distance)| {
            // Use the quadratic formula to solve for the start time that would achieve the record.
            let (t, d) = (time as f64, distance as f64);
            let s = (t - (t * t - 4.0 * d).sqrt()) / 2.0;
            time + 1 - (s as i64 + 1) * 2
        })
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
