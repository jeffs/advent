use crate::count::count;

fn parse_line(line: &str) -> i64 {
    line[line.find(':').expect("colon") + 1..]
        .trim()
        .replace(' ', "")
        .parse()
        .expect("number")
}

pub fn solve(text: &str) -> i64 {
    let (time, distance) = text.split_once('\n').expect("two input lines");
    let (time, distance) = (parse_line(time), parse_line(distance));
    count(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve(include_str!("sample.txt")), 71503);
    }
}
